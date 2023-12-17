use core::{arch::asm, mem};

use crate::{sbi::shutdown, trap::TrapContext};

pub const MAX_APP_NUM: usize = 16;
pub const APP_BASE_ADDRESS: usize = 0x8040_0000;
pub const APP_SIZE_LIMIT: usize = 0x20_000;

pub const KERNRL_STACK_SIZE: usize = 4096 * 2;
pub const USER_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
#[derive(Clone, Copy)]
struct KernelStack {
    data: [u8; KERNRL_STACK_SIZE],
}
impl KernelStack {
    fn get_sp(&self) -> usize {
        return self.data.as_ptr() as usize + KERNRL_STACK_SIZE;
    }
    pub fn push_context(&self, cx: TrapContext) -> usize {
        let cx_ptr = (self.get_sp() - mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        cx_ptr as usize
    }
}

#[repr(align(4096))]
#[derive(Clone, Copy)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn get_sp(&self) -> usize {
        return self.data.as_ptr() as usize + USER_STACK_SIZE;
    }
}
static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNRL_STACK_SIZE],
}; MAX_APP_NUM];

static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn get_base_i(id: usize) -> usize {
    APP_BASE_ADDRESS + id * APP_SIZE_LIMIT
}

pub fn init_app_cx(id: usize) -> usize {
    KERNEL_STACK[id].push_context(TrapContext::app_init_context(
        get_base_i(id),
        USER_STACK[id].get_sp(),
    ))
}

pub fn load_apps() {
    extern "C" {
        fn _num_app();
    }

    let num_app_prt = _num_app as usize as *const usize;
    let num_app = get_num_app();

    let app_start = unsafe { core::slice::from_raw_parts(num_app_prt.add(1), num_app + 1) };

    unsafe {
        asm!("fence.i");
    }

    for i in 0..num_app {
        let base_i = get_base_i(i);
        (base_i..base_i + APP_SIZE_LIMIT).for_each(|addr| unsafe {
            (addr as *mut u8).write_volatile(0);
        });
        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
        };

        let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };
        dst.copy_from_slice(src);
    }
}
