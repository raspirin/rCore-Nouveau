#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod batch;
mod console;
mod lang_items;
mod logging;
mod sbi;
mod sync;
mod syscall;
mod trap;

use crate::sbi::shutdown;
use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    trap::init();
    batch::init();
    println!("[kernel] hello, world");
    batch::run_next_app();
    shutdown();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
