#![no_std]
#![no_main]
#![feature(naked_functions)]

#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".text._start"]
unsafe extern "C" fn _start() -> ! {
  core::arch::asm! {
    "b 1f",
    ".space 0xE0",
    "1:",
    "mov r0, #0x04000000",
    "mov r1, #0",
    "strh r1, [r0]",
    "2:",
    "b 2b",
    options(noreturn)
  }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
