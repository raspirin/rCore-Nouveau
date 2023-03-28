#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod batch;
mod console;
mod lang_items;
mod logging;
mod sbi;
mod sync;

use crate::sbi::shutdown;
use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    println!("[kernel] hello, world");
    shutdown();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
