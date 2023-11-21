#![allow(unused)]

use core::arch::asm;
pub const SBI_SET_TIMER: usize = 0;
pub const SBI_CONSOLE_PUTCHAR: usize = 1;
pub const SBI_CONSOLE_GETCHAR: usize = 2;
pub const SBI_CLEAR_IPI: usize = 3;
pub const SBI_SEND_IPI: usize = 4;
pub const SBI_REMOTE_FENCE_I: usize = 5;
pub const SBI_REMOTE_SFENCE_VMA: usize = 6;
pub const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
pub const SBI_SHUTDOWN: usize = 8;
pub const SYSCALL_EXIT: usize = 93;

#[inline(always)]
pub fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm! {
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which
        }
    };
    ret
}

pub fn consle_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn exit(code: i32) {
    sbi_call(SYSCALL_EXIT, code as usize, 0, 0);
}

pub fn shutdown(message: Option<&str>) {
    if let Some(msg) = message {
        crate::println!("{}", msg);
    };
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    exit(0);
}
