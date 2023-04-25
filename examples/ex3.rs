#![no_std]
#![no_main]

use gba_from_scratch::{Color, DisplayControl, BACKDROP, DISPCNT, OBJ_PALETTE};

const JUST_SHOW_OBJECTS: DisplayControl =
  DisplayControl::new().with_objects(true);

#[no_mangle]
pub extern "C" fn main() -> ! {
  BACKDROP.write(Color::MAGENTA);
  OBJ_PALETTE.index(1).write(Color::RED);
  OBJ_PALETTE.index(2).write(Color::WHITE);

  DISPCNT.write(JUST_SHOW_OBJECTS);

  loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
