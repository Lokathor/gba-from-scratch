#![no_std]
#![no_main]

fn main() {
  unsafe {
    (0x0400_0000 as *mut u16).write_volatile(0);
  }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
