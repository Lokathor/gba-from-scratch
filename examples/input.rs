#![no_std]
#![no_main]

extern crate gba_from_scratch;

#[no_mangle]
extern "C" fn main() -> ! {
  unsafe {
    (0x0500_0000 as *mut u16).write_volatile(0b11111);
    (0x0400_0000 as *mut u16).write_volatile(0);
  }
  loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[link_section = ".iwram"]
pub static VALUE: i32 = 5;
