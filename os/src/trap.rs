use core::arch::global_asm;

use riscv::register::{
    scause,
    scause::Exception,
    sstatus::{self, Sstatus, SPP},
    stval, stvec,
    utvec::TrapMode,
};

use crate::syscall::syscall;

global_asm!(include_str!("trap.S"));

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp
    }
    pub fn app_init_context(sepc: usize, sp: usize) -> Self {
        let sstatus = sstatus::read();
        unsafe { sstatus::set_spp(SPP::User) };
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc,
        };
        cx.set_sp(sp);
        cx
    }
}

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe { stvec::write(__alltraps as usize, TrapMode::Direct) }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        scause::Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        scause::Trap::Exception(Exception::StoreFault)
        | scause::Trap::Exception(Exception::StorePageFault) => {
            println!("[Helium]: Exception::StorePageFault | Exception::StoreFault");
        }
        scause::Trap::Exception(Exception::IllegalInstruction) => {
            println!("[Helium]: Exception::IllegalInstruction");
        }
        _ => {
            panic!("unsupport trap {:?},  stval= {:#x}!", scause.cause(), stval);
        }
    }
    cx
}
