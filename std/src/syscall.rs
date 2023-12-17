#![allow(unused)]

use core::arch::asm;

pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;
pub const SYS_YIELD: usize = 124;

#[inline(always)]
pub fn syscall(which: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
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

pub fn sys_write(fd: usize, buffer: &[u8]) {
    syscall(SYS_WRITE, fd, buffer.as_ptr() as usize, buffer.len());
}

pub fn sys_exit(code: i32) {
    syscall(SYS_EXIT, code as usize, 0, 0);
}

pub fn sys_yield() -> isize {
    syscall(SYS_YIELD, 0, 0, 0)
}
