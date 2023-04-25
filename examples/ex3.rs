#![no_std]
#![no_main]

use gba_from_scratch::{Color, DisplayControl, BACKDROP, DISPCNT, KEYINPUT};

const JUST_SHOW_OBJECTS: DisplayControl =
  DisplayControl::new().with_objects(true);

#[no_mangle]
pub extern "C" fn main() -> ! {
  DISPCNT.write(JUST_SHOW_OBJECTS);

  loop {
    let k = KEYINPUT.read();
    BACKDROP.write(if k.a() { Color::RED } else { Color::GREEN })
  }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
