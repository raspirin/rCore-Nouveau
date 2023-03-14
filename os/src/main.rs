#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod console;
mod lang_items;
mod sbi;

use core::arch::{asm, global_asm};
use crate::sbi::shutdown;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    shutdown();
    //loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
