use core::arch::global_asm;
use riscv::register::mtvec::TrapMode;
use riscv::register::{scause, stval, stvec};
use riscv::register::scause::{Trap, Exception};

mod context;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct)
    }
}

pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        _ => todo!()
    }
    cx
}

pub use context::TrapContext;