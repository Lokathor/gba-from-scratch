#![no_std]
#![feature(naked_functions)]

use bitfrob::u16_with_bit;
use voladdress::{Safe, VolAddress, VolBlock, VolSeries};

macro_rules! kilobytes {
  ($bytes:expr) => {
    $bytes * 1024
  };
}

pub const DISPCNT: VolAddress<DisplayControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0000) };

pub const KEYINPUT: VolAddress<KeyInput, Safe, ()> =
  unsafe { VolAddress::new(0x400_0130) };

pub const BACKDROP: VolAddress<Color, Safe, Safe> =
  unsafe { VolAddress::new(0x0500_0000) };

pub const OBJ_PALETTE: VolBlock<Color, Safe, Safe, 256> =
  unsafe { VolBlock::new(0x0500_0200) };

pub const PIXELS_PER_TILE: usize = 8 * 8;
pub const BITS_PER_BYTE: usize = 8;
pub const SIZE_OF_TILE4: usize = (PIXELS_PER_TILE * 4) / BITS_PER_BYTE;
pub const SIZE_OF_TILE8: usize = (PIXELS_PER_TILE * 8) / BITS_PER_BYTE;
pub const SIZE_OF_OBJ_TILE_MEM: usize = kilobytes!(32);
pub const SIZE_OF_U32: usize = core::mem::size_of::<u32>();
pub const TILE4_WORD_COUNT: usize = SIZE_OF_TILE4 / SIZE_OF_U32;
pub const TILE8_WORD_COUNT: usize = SIZE_OF_TILE8 / SIZE_OF_U32;
pub const OBJ_TILE_MEM_WORD_COUNT: usize = SIZE_OF_OBJ_TILE_MEM / SIZE_OF_U32;

pub const OBJ_TILES_U32: VolBlock<u32, Safe, Safe, OBJ_TILE_MEM_WORD_COUNT> =
  unsafe { VolBlock::new(0x0601_0000) };

pub type Tile4 = [u32; TILE4_WORD_COUNT];
pub const OBJ_TILE4: VolBlock<Tile4, Safe, Safe, 1024> =
  unsafe { VolBlock::new(0x0601_0000) };

pub type Tile8 = [u32; TILE8_WORD_COUNT];
pub const OBJ_TILE8: VolSeries<Tile8, Safe, Safe, 1023, 32> =
  unsafe { VolSeries::new(0x0601_0000) };

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr0(pub u16);

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr1(pub u16);

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr2(pub u16);

pub const OBJ_ATTRS_0: VolSeries<ObjAttr0, Safe, Safe, 128, 64> =
  unsafe { VolSeries::new(0x0700_0000) };
pub const OBJ_ATTRS_1: VolSeries<ObjAttr1, Safe, Safe, 128, 64> =
  unsafe { VolSeries::new(0x0700_0000 + 2) };
pub const OBJ_ATTRS_2: VolSeries<ObjAttr2, Safe, Safe, 128, 64> =
  unsafe { VolSeries::new(0x0700_0000 + 4) };

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ObjAttr(pub ObjAttr0, pub ObjAttr1, pub ObjAttr2);

pub const OBJ_ATTRS: VolSeries<ObjAttr, Safe, Safe, 128, 64> =
  unsafe { VolSeries::new(0x0700_0000) };

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Color(pub u16);
impl Color {
  pub const BLACK: Self = Self::rgb(0, 0, 0);
  pub const BLUE: Self = Self::rgb(0, 0, 31);
  pub const GREEN: Self = Self::rgb(0, 31, 0);
  pub const CYAN: Self = Self::rgb(0, 31, 31);
  pub const RED: Self = Self::rgb(31, 0, 0);
  pub const MAGENTA: Self = Self::rgb(31, 0, 31);
  pub const YELLOW: Self = Self::rgb(31, 31, 0);
  pub const WHITE: Self = Self::rgb(31, 31, 31);

  #[inline]
  #[must_use]
  pub const fn rgb(r: u16, g: u16, b: u16) -> Self {
    Self(r | (g << 5) | (b << 10))
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct KeyInput(pub u16);
#[rustfmt::skip]
impl KeyInput {
  #[inline]
  pub const fn a(self) -> bool { (self.0 & (1<<0)) == 0 }
  #[inline]
  pub const fn b(self) -> bool { (self.0 & (1<<1)) == 0 }
  #[inline]
  pub const fn select(self) -> bool { (self.0 & (1<<2)) == 0 }
  #[inline]
  pub const fn start(self) -> bool { (self.0 & (1<<3)) == 0 }
  #[inline]
  pub const fn right(self) -> bool { (self.0 & (1<<4)) == 0 }
  #[inline]
  pub const fn left(self) -> bool { (self.0 & (1<<5)) == 0 }
  #[inline]
  pub const fn up(self) -> bool { (self.0 & (1<<6)) == 0 }
  #[inline]
  pub const fn down(self) -> bool { (self.0 & (1<<7)) == 0 }
  #[inline]
  pub const fn r(self) -> bool { (self.0 & (1<<8)) == 0 }
  #[inline]
  pub const fn l(self) -> bool { (self.0 & (1<<9)) == 0 }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DisplayControl(u16);
impl DisplayControl {
  #[inline]
  pub const fn new() -> Self {
    Self(0)
  }
  #[inline]
  pub const fn with_linear_obj_tiles(self, linear: bool) -> Self {
    Self(u16_with_bit(6, self.0, linear))
  }
  #[inline]
  pub const fn with_forced_blank(self, blank: bool) -> Self {
    Self(u16_with_bit(7, self.0, blank))
  }
  #[inline]
  pub const fn with_objects(self, objects: bool) -> Self {
    Self(u16_with_bit(12, self.0, objects))
  }
}

#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".text._start"]
unsafe extern "C" fn _start() -> ! {
  core::arch::asm! {
    "b 1f",
    ".space 0xE0",
    "1:",
    "ldr r12, =main",
    "bx r12",
    options(noreturn)
  }
}
