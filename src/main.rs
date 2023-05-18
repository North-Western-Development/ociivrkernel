#![no_std]
#![no_main]
#![feature(naked_functions, panic_info_message)]

use core::{panic::PanicInfo, ptr::write_volatile};

#[naked]
#[no_mangle]
#[link_section = ".text.init"]
unsafe extern "C" fn _start() -> ! {
    use core::arch::asm;
    asm!(
      // before we use the `la` pseudo-instruction for the first time,
      //  we need to set `gp` (google linker relaxation)
      ".option push",
      ".option norelax",
      "la gp, _global_pointer",
      ".option pop",

      // set the stack pointer
      "la sp, _init_stack_top",

      // "tail-call" to {entry} (call without saving a return address)
      "tail {entry}",
      entry = sym entry, // {entry} refers to the function [entry] below
      options(noreturn) // we must handle "returning" from assembly
    );
}

extern "C" fn entry() -> ! {
    unsafe { uart::init_console(0x1000_0148) };

    print!("\x1b[2J");

    print!("\x1b[0;0H");

    print!("\x1b[1;31mHello world\x1b[0m");

    print!("\x1b[2;0H");
    //println!("Hello world");
    loop {}
}

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    println!("\x1b[1;31mPanic: {}\x1b[0m", info.message().unwrap());
    loop {}
}

mod uart;
