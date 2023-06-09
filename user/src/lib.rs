#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

use syscall::*;

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Can't find main!");
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}