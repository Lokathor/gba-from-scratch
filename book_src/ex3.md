# Objects

Now that we can get user input there's a *lot* of things that we could learn about next.
Probably we should focus on how to improve our drawing abilities.

Most of the GBA's drawing abilities involve either the 4 background layers, or the 128 objects (called "OBJ" for short).
The background layers let you draw a few "big" things (128x128 or bigger), and the objects let you draw many "small" things (64x64 or less).

The objects have a consistent behavior, while the four background layers behave differently depending on the "video mode" that you set in the display control.
That's reason enough to focus on the objects first.

## Display Control

We've already seen that the display control has a "forced blank" bit.
Most of the other bits are for background control stuff, but since some of them affect object display we'll just cover that right now.

| Bit(s) | Setting |
|:-:|:-|
| 0-2 | Video Mode |
| 3   | (Unused in GBA mode) |
| 4   | Frame Select |
| 5   | Unlocked H-blank |
| 6   | Linear object tile mapping |
| 7   | Forced Blank |
| 8   | Enable Background 0 |
| 9   | Enable Background 1 |
| 10  | Enable Background 2 |
| 11  | Enable Background 3 |
| 12  | Enable Objects |
| 13  | Window 0 Display Flag |
| 14  | Window 1 Display Flag |
| 15  | OBJ Window Display Flag |

* **Video Mode:** This sets which mode the four background layers will operate with. Despite this being a 3-bit field, only modes 0 through 5 give a useful display. Modes 6 and 7 cause garbage output.
* **Frame Select:** Affects which bitmap frame is used in video mode 4 or 5.
* **Unlocked H-blank:** GBATEK calls this "H-Blank Interval Free", and mGBA's debug controls call this "Unlocked H-blank". This bit affects what you can do during the "horizontal blank" time between each scanline being shown, but when it's on fewer objects can be drawn. We won't be doing any per-scanline drawing for now, so we'll leave it off by default.
* **Linear object tile mapping:** This affects how we lay out the tiles for multi-tile objects. We'll talk about the details of this in just a moment.
* **Forced Blank:** Hey we know about this bit. When it's on, the display won't access any memory and will just output white pixels any time it would have rendered a pixel normally.
* **Enable Background:** These four bits set if we want each of the four background layers on. For now we don't care.
* **Enable Objects:** This bit sets the objects to be displayed.
* **Window Flags:** These three bits affect the "window" special graphical feature. We'll ignore these bits for now.

I'm going to use the `bitfrob` crate to get some bit manipulation utilities.

```
> cargo add bitfrob
    Updating crates.io index
      Adding bitfrob v1.3.0 to dependencies.
             Features:
             - track_caller
    Updating crates.io index
```

Now we can give a type to our display control value, as well as just enough methods to get started.
Unlike with our `Color` type, with the `DisplayControl` we want to completely prevent an invalid video mode from being set, so we'll keep the `u16` that we're wrapping as a private field.
Then we just have one "builder" method for each bit or group of bits that we want to be able to change.
To start we can skip all the background related bits, so we'll only need three builders.

```rust
// in lib.rs

use bitfrob::u16_with_bit;

pub const DISPCNT: VolAddress<DisplayControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0000) };

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
```

This will require updates to both `ex2.rs` and `ex3.rs`.
* For example 2, instead of writing `0` we'd write `DisplayControl::new()` instead.
* For example 3, we want to enable object display, since we're about to start showing some objects.

```rust
// in ex3.rs

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
```

That's it for the display control.

## Object Palette

TODO

## Object Tile Memory

TODO

## Object Attribute Memory

TODO

## Waiting For Vertical Blank

TODO
