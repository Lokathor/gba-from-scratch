#![no_std]
#![feature(naked_functions)]

use voladdress::{Safe, VolAddress};

pub const DISPCNT: VolAddress<u16, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0000) };

pub const KEYINPUT: VolAddress<KeyInput, Safe, ()> =
  unsafe { VolAddress::new(0x400_0130) };

pub const BACKDROP: VolAddress<Color, Safe, Safe> =
  unsafe { VolAddress::new(0x0500_0000) };

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Color(pub u16);
impl Color {
  pub const RED: Self = Self::rgb(31, 0, 0);
  pub const GREEN: Self = Self::rgb(0, 31, 0);

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
