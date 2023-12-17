#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;

// #[macro_use]
// extern crate lazy_static;

#[macro_use]
mod stdout;
mod loader;
mod sync;
mod syscall;
mod task;
mod trap;
use core::arch::global_asm;

use crate::task::run_first;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    println!("====[Helium]=================================");
    trap::init();
    loader::load_apps();
    run_first();
    // sbi::shutdown(Some("Shutdown os"));
}

pub fn clear_bss() {
    extern "C" {
        // defined in linker.ld
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
