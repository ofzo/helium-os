#![no_std]
#![no_main]

#[macro_use]
extern crate std;

use riscv::register::sstatus;

#[no_mangle]
fn main() -> i32 {
    println!("Try to access privileged CSR in U Mode");
    println!("Kernel should kill this application");
    unsafe {
        sstatus::set_spp(sstatus::SPP::User);
    }
    0
}
