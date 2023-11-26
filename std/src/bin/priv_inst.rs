#![no_std]
#![no_main]

#[macro_use]
extern crate std;

use core::arch::asm;

#[no_mangle]
fn main() -> i32 {
    println!("Try to execute privileged instruction in U Mode");
    println!("Kenel should kill this application!");
    unsafe {
        asm!("sret");
    }
    0
}
