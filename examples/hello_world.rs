#![no_std]
#![no_main]

extern crate alloc;
use hopter::{debug::semihosting::dbg_println, task::main};

/// CPUID register address
const CPUID: *const u32 = 0xE000_ED00 as *const u32;

fn detect_core() {
    let cpuid = unsafe { core::ptr::read_volatile(CPUID) };

    let part = (cpuid >> 4) & 0xFFF;

    match part {
        0xC27 => dbg_println!("Running on Cortex-M7!"),
        0xC24 => dbg_println!("Running on Cortex-M4!"),
        _ => dbg_println!("Running on Unknown Core!"),
    }
}

// Attribute `#[main]` marks the function as the entry function for the main
// task. The function name can be arbitrary. The main function should accept
// one argument which is the Cortex-M core peripherals.s
#[main]
fn main(_: cortex_m::Peripherals) {
    detect_core();

    // Print via semihosting. When using QEMU with semihosting option enabled,
    // the characters will appear on the QEMU console.
    dbg_println!("hello world!");

    loop {}
}
