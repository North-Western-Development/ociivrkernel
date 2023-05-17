#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::panic::PanicInfo;

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
    let mut console = unsafe { uart::Device::new(0x1000_0148) };
    for byte in "Hello, world!".bytes() {
        console.put(byte);
    }
    loop {}
}

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    loop {}
}

mod uart;
