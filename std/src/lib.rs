#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
mod stdout;
mod lang_items;
mod syscall;

pub use stdout::print;
pub use syscall::sys_yield;
pub use syscall::{sys_exit as exit, sys_write as write};

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

pub fn clear_bss() {
    extern "C" {
        // defined in linker.ld
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize)
        .for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
