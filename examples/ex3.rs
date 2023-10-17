#![no_std]
#![no_main]

use gba_from_scratch::{
  Color, DisplayControl, ObjAttr, BACKDROP, DISPCNT, OBJ_ATTRS, OBJ_PALETTE,
  OBJ_TILE4,
};

const JUST_OBJECTS_LINEAR: DisplayControl =
  DisplayControl::new().with_objects(true).with_linear_obj_tiles(true);

#[no_mangle]
pub extern "C" fn main() -> ! {
  BACKDROP.write(Color::MAGENTA);
  OBJ_PALETTE.index(1).write(Color::RED);
  OBJ_PALETTE.index(2).write(Color::WHITE);

  OBJ_TILE4.index(1).write(TILE_UP_LEFT);
  OBJ_TILE4.index(2).write(TILE_UP_RIGHT);
  OBJ_TILE4.index(3).write(TILE_DOWN_LEFT);
  OBJ_TILE4.index(4).write(TILE_DOWN_RIGHT);

  let obj = ObjAttr::new().with_size(1).with_tile(1).with_x(10).with_y(23);
  OBJ_ATTRS.index(0).write(obj);

  DISPCNT.write(JUST_OBJECTS_LINEAR);

  loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

/// A tile with an extra notch on the upper left.
#[rustfmt::skip]
const TILE_UP_LEFT: [u32; 8] = [
  0x11111111,
  0x12222111,
  0x12222111,
  0x12222221,
  0x12222221,
  0x12222221,
  0x12222221,
  0x11111111,
];

/// A tile with an extra notch on the upper right.
#[rustfmt::skip]
const TILE_UP_RIGHT: [u32; 8] = [
  0x11111111,
  0x11122221,
  0x11122221,
  0x12222221,
  0x12222221,
  0x12222221,
  0x12222221,
  0x11111111,
];

/// A tile with an extra notch on the lower left.
#[rustfmt::skip]
const TILE_DOWN_LEFT: [u32; 8] = [
  0x11111111,
  0x12222221,
  0x12222221,
  0x12222221,
  0x12222221,
  0x12222111,
  0x12222111,
  0x11111111,
];

/// A tile with an extra notch on the lower right.
#[rustfmt::skip]
const TILE_DOWN_RIGHT: [u32; 8] = [
  0x11111111,
  0x12222221,
  0x12222221,
  0x12222221,
  0x12222221,
  0x11122221,
  0x11122221,
  0x11111111,
];
