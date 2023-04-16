#![no_std]
#![no_main]

fn main() {
  //
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
