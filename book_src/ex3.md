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

For now that's all we need to do for the display control.

## Object Palette

Objects always need to use "paletted" color.
Instead of each pixel within the object's image holding a full color value, it just holds an index into the palette.
This allows each pixel to only need 4 or 8 bits each, instead of the 16 bits needed for a complete color.

The palette for objects starts at `0x0500_0200`, and it's 256 entries long.
Each object can use 8 bits per pixel (8bpp) or 4 bits per pixel (4bpp).

* When an object is set for 8bpp each non-zero pixel value is the 8-bit index into the object palette.
  A pixel value of 0 means that the object is transparent in that pixel.
  This allows for up to 255 colors to be used within a single object.
* When an object is set for 4bpp each non-zero pixel value is *the low half* of the full index value.
  A second setting within the object's attributes determine the upper half of the index value.
  This effectively splits the palette memory into 16 "palbank" groupings.
  As with 8bpp objects, a pixel value of 0 makes a transparent pixel.
  This allows for up to 15 colors within a single object.

You might notice that index 0 of the object palette isn't ever used by either mode.
The memory itself exists for consistency, but the GBA will never use the color value in that position.
Call it a free global variable for your own personal use, if you want.

Since we have a series of color values instead of just a single color value,
this time we'll declare the object palette as a [VolBlock](https://docs.rs/voladdress/latest/voladdress/struct.VolBlock.html) instead of a `VolAddress`.

```rust
// in lib.rs

pub const OBJ_PALETTE: VolBlock<Color, Safe, Safe, 256> =
  unsafe { VolBlock::new(0x0500_0200) };
```

A `VolBlock` works mostly like an array does.
We call `OBJ_PALETTE.index(i)` to get a particular `VolAddress`, and then we can read or write that address.
We could also use `get` if we want to do an optional lookup, or we could iterate the block, etc.

First let's make some more named color constants.
We'll name each of the 8 colors you get when each of the three color channels is either no-intensity or full-intensity.

```rust
// in lib.rs

impl Color {
  pub const BLACK: Self = Self::rgb(0, 0, 0);
  pub const BLUE: Self = Self::rgb(0, 0, 31);
  pub const GREEN: Self = Self::rgb(0, 31, 0);
  pub const CYAN: Self = Self::rgb(0, 31, 31);
  pub const RED: Self = Self::rgb(31, 0, 0);
  pub const MAGENTA: Self = Self::rgb(31, 0, 31);
  pub const YELLOW: Self = Self::rgb(31, 31, 0);
  pub const WHITE: Self = Self::rgb(31, 31, 31);
  // ...
}
```

Now we can set up a backdrop color and two different palette entries.

```rust
// in ex3.rs

#[no_mangle]
pub extern "C" fn main() -> ! {
  BACKDROP.write(Color::MAGENTA);
  OBJ_PALETTE.index(1).write(Color::RED);
  OBJ_PALETTE.index(2).write(Color::WHITE);

  DISPCNT.write(JUST_SHOW_OBJECTS);

  loop {}
}
```

If we run the example in mGBA we can check our work using the debug utilities.
In the menu, "Tools -> Game State Views -> View Palette..." will open a dialog showing all the background and object palette info.

* The backdrop color will show up in the 0th entry of the background palette.
* The two object palette colors will be in positions 1 and 2 of the top row.

Each row of the palette is shown 16 colors at a time, so it's easy to tell what's happening in both 8bpp and 4bpp modes.

That should be enough palette setup to continue with the tutorial.

## Object Tile Memory

TODO

## Object Attribute Memory

TODO

## Waiting For Vertical Blank

TODO
