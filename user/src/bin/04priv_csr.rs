#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use riscv::register::sstatus;
use riscv::register::sstatus::SPP;

#[no_mangle]
fn main() -> i32 {
    println!("Try to access privileged CSR in U Mode");
    println!("Kernel should kill this program!");
    unsafe {
        sstatus::set_spp(SPP::User);
    }
    0
}
