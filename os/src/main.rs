#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod console;
mod lang_items;
mod logging;
mod sbi;

use crate::sbi::shutdown;
use core::arch::global_asm;
use log::{debug, error, info, trace, warn};
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    println!("Hello, world!");
    warn!("This is a warn message.");
    error!("This is an error message.");
    debug!("This is a debug message.");
    info!("This is an info message");
    trace!("This is an trace message");
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
