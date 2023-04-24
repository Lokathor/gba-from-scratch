#![no_std]
#![no_main]

use gba_from_scratch::{Color, BACKDROP, DISPCNT, KEYINPUT};

#[no_mangle]
pub extern "C" fn main() -> ! {
  DISPCNT.write(0);
  loop {
    let k = KEYINPUT.read();
    BACKDROP.write(if k.a() { Color::RED } else { Color::GREEN })
  }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
