use core::{arch::asm, mem};

use lazy_static::*;

use crate::{sbi::shutdown, sync::UPSafeCell, trap::TrapContext};

const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x8040_0000;
const APP_SIZE_LIMIT: usize = 0x20_000;

const KERNRL_STACK_SIZE: usize = 4096 * 2;
const USER_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNRL_STACK_SIZE],
}
impl KernelStack {
    fn get_sp(&self) -> usize {
        return self.data.as_ptr() as usize + KERNRL_STACK_SIZE;
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn get_sp(&self) -> usize {
        return self.data.as_ptr() as usize + USER_STACK_SIZE;
    }
}
static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNRL_STACK_SIZE],
};

static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_info(&self) {
        println!("[Helium]: num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[Helium]: app_{} [{:#x}, {:#x}]",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }
    pub fn current_app(&self) -> usize {
        return self.current_app;
    }
    pub fn next_app(&mut self) {
        self.current_app += 1;
    }
    unsafe fn load_app(&self, id: usize) {
        if id >= self.num_app {
            println!("[Helium]: all application complete!");
            shutdown(false);
        }
        println!("[Helium]: app_{} loading", id);
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = core::slice::from_raw_parts(
            self.app_start[id] as *const u8,
            self.app_start[id + 1] - self.app_start[id],
        );
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
        asm!("fence.i");
    }
}

lazy_static! {
    static ref APP_MAMAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

pub fn init() {
    APP_MAMAGER.access().print_info();
}

pub fn run_next_app() -> ! {
    let mut app = APP_MAMAGER.access();
    let current = app.current_app();
    unsafe {
        app.load_app(current);
    }
    app.next_app();
    drop(app);
    extern "C" {
        fn __restore(addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    panic!("unreachable  in batch::run_cruuent_app")
}
