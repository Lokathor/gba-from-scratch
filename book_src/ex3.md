# Objects

Now that we can get user input there's a *lot* of things that we could learn about next.
Probably we should focus on how to improve our drawing abilities.

Most of the GBA's drawing abilities involve either the 4 background layers, or the 128 objects (called "OBJ" for short).
The background layers let you draw a few "big" things (128x128 or bigger), and the objects let you draw many "small" things (64x64 or less).

The objects have a fairly consistent behavior, while the four background layers behave differently depending on the "video mode" that you set in the display control.
That's reason enough to focus on the objects first.

Small additonal note: "objects" are sometimes called "sprites" too.
Depending on who you ask the two words are either completely interchangable *or* they have important differences.
What exactly the difference is can depend on who you ask.
I don't really care myself, so I'm just going to stick to "objects" as often as I can in this tutorial and not worry about it.

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

First, what is a tile exactly:

* A tile is an 8x8 square of palette indexes.
* A palette index can be either 4 bits or 8 bits (the "bit depth").
* The indexes store one row at a time, left to right, top to bottom.

So we might have the following Rust constants

```rust
// in lib.rs

pub const PIXELS_PER_TILE: usize = 8 * 8;
pub const BITS_PER_BYTE: usize = 8;
pub const SIZE_OF_TILE4: usize = (PIXELS_PER_TILE * 4) / BITS_PER_BYTE;
pub const SIZE_OF_TILE8: usize = (PIXELS_PER_TILE * 8) / BITS_PER_BYTE;
```

Also, there's 32K of object tile RAM.

```rust
// in lib.rs

macro_rules! kilobytes {
  ($bytes:expr) => {
    $bytes * 1024
  };
}

pub const SIZE_OF_OBJ_TILE_MEM: usize = kilobytes!(32);
```

Now we know how bit everything is, in bytes.
However, video memory doesn't work right with byte writes.
We can cover the details another time, but with video memory you always have to write in 16-bit or 32-bit chunks.
Also, the GBA is simply much faster at transferring bulk data around when it's aligned to 4.
Data aligned to 4 can be copied one or more `u32` values at time (one or more "words" in ARM terms).
Being more aligned than 4 doesn't help any extra, but we want to have at least alignment 4 with anything big.
Tiles, particularly if we've got dozens or hundreds of them, count as "big enough to care about alignment".
So we'll model tile data as being arrays of `u32` values, which will keep the data aligned to 4.

```rust
// in lib.rs

pub const SIZE_OF_U32: usize = core::mem::size_of::<u32>();
pub const TILE4_WORD_COUNT: usize = SIZE_OF_TILE4 / SIZE_OF_U32;
pub const TILE8_WORD_COUNT: usize = SIZE_OF_TILE8 / SIZE_OF_U32;
pub const OBJ_TILE_MEM_WORD_COUNT: usize = SIZE_OF_OBJ_TILE_MEM / SIZE_OF_U32;
```

Which lets us declare the block of *words* where our object tile data goes.

```rust
// in lib.rs

pub const OBJ_TILES_U32: VolBlock<u32, Safe, Safe, OBJ_TILE_MEM_WORD_COUNT> =
  unsafe { VolBlock::new(0x0601_0000) };
```

Here's where things get kinda weird.
An object's attributes (most of which we'll cover lower down) include a "Tile ID" for the base tile of the object.
These tile id values are used as an index for 32 byte offsetting, regardless of 4bpp or 8bpp.
This means that they line up perfectly with a 4bpp view of the tile data, and we get 1024 IDs.

```rust
// in lib.rs

pub type Tile4 = [u32; TILE4_WORD_COUNT];
pub const OBJ_TILE4: VolBlock<Tile4, Safe, Safe, 1024> =
  unsafe { VolBlock::new(0x0601_0000) };
```

But with 8bpp objects we end up in a pickle.
We could use a [VolSeries](https://docs.rs/voladdress/latest/voladdress/struct.VolSeries.html), which is an alternative to the `VolBlock` type, for when the stride and the element size aren't the same.
The `VolSeries` type is mostly intended for when the stride is *bigger* than the element size, but the math will work out either way.
Note that since 8bpp tiles are twice as big we have to cut down the number of tiles from 1024 to 1023 so that using the last index doesn't go out of bounds.

```rust
// in lib.rs

pub type Tile8 = [u32; TILE8_WORD_COUNT];
pub const OBJ_TILE8: VolSeries<Tile8, Safe, Safe, 1023, 32> =
  unsafe { VolSeries::new(0x0601_0000) };
```

And, well, it looks kinda weird every time I look at the code but... that's how the hardware works.
It's the ultimate arbiter of what's correct, so sometimes you gotta just go with it.

We can always think about this more later, and maybe improve it then.
For now it's enough that we've got the right addresses at all.

**One final note:** In video modes 3, 4, and 5 the lower half of the object tile region instead gets used as part of the background.
In this case, only object tile index values 512 and above are usable for object display.

## Object Attribute Memory

Separate from the object tile memory, there's also the Object Attribute Memory (OAM) region.
This has space for 128 "attribute" entries, which defines how the objects are shown.
An object's "attribute" data is 6 bytes, split into three `u16` values.
The three fields don't have fancy names, they're just called 0, 1, and 2.

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr0(pub u16);

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr1(pub u16);

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjAttr2(pub u16);
```

In between each attribute data entry is *part of* an affine entry.
That's right, only a part of one.
A full affine entry is four `i16` values (called A, B, C, and D).
There's one `i16` affine value per three `u16` attribute values.
The memory looks kinda like this.

* obj0.attr0
* obj0.attr1
* obj0.attr2
* affine0.a
* obj1.attr0
* obj1.attr1
* obj1.attr2
* affine0.b
* obj2.attr0
* obj2.attr1
* obj2.attr2
* affine0.c
* obj3.attr0
* obj3.attr1
* obj3.attr2
* affine0.d

And so on, so that spread among the 128 object attribute entries there's also 32 affine entries.
It's a little strange.
It's just... how it works.

Once again we'll use a `VolSeries` to model this.
We can make one declaration per attribute series.
Attribute 0 will be at the "base" of the OAM region,
with attribute 1 being offset by 2 bytes (the size of a `u16`),
and attribute 2 being offset by 4 bytes (the size of two `u16`).

```rust
// in lib.rs

pub const OBJ_ATTRS_0: VolSeries<ObjAttr0, Safe, Safe, 128, 64> =
  unsafe { VolSeries::new(0x0700_0000) };
pub const OBJ_ATTRS_1: VolSeries<ObjAttr1, Safe, Safe, 128, 64> =
  unsafe { VolSeries::new(0x0700_0000 + 2) };
pub const OBJ_ATTRS_2: VolSeries<ObjAttr2, Safe, Safe, 128, 64> =
  unsafe { VolSeries::new(0x0700_0000 + 4) };
```

Alternately, we could group the attributes into a single struct and view things that way.

```rust
// in lib.rs

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ObjAttr(pub ObjAttr0, pub ObjAttr1, pub ObjAttr2);

pub const OBJ_ATTRS: VolSeries<ObjAttr, Safe, Safe, 128, 64> =
  unsafe { VolSeries::new(0x0700_0000) };
```

This is definitely one of those "it depends on how you want to do it" things.

### Object Attribute 0

| Bit(s) | Setting |
|:-:|:-|
| 0-7  | Y coordinate (8 bits) |
| 8    | Affine flag |
| 9    | Double size (affine) OR invisible (non-affine) |
| 10-11| Mode: Normal, Semi-transparent, Window |
| 12   | Mosaic flag |
| 13   | Use 8bpp |
| 14-15| Orientation: Square, Horizontal, Vertical |

The Y coordinate is explained in the "Object Positioning", below.

The Affine flag in Bit 8 determines if the object will use affine display.
We're going to leave all discussion of affine drawing for a later lesson.

Bit 9 can make an affine object double size, or it can make a non-affine object invisible.
It might be easier to think of bits 8 and 9 as being "one value" which sets one of four styles,
but mGBA's debug viewer shows them as two separate flags so I'm going to list them as two separate flags.

The objects's Mode sets which special effect the object should be part of.
We'll talk about blending effects and the window effect later on, but now you know what bits are involed on the object side.

Similarly, you can have an object enable the mosaic effect, but special effects are some future lesson.

The 8bpp flag sets if the object's tile data should be interpreted as 8bpp or 4bpp.
Each object can decide for itself which mode to use.

Finally, each object gets an "Orientation", which GBATEK calls the "Shape".
This is combined with the object's Size (see below) to determine the object's dimensions.

### Object Attribute 1

| Bit(s) | Setting |
|:-:|:-|
| 0-8  | X coordinate (9 bits) |
| 9-13 | Affine entry index (affine only) |
| 12   | Horizontal flip flag (non-affine) |
| 13   | Vertical flip flag (non-affine) |
| 14-15| Size: 0 to 4 (see below) |

The X coordinate is explained in the "Object Positioning", below.

The Affine Entry index determines which affine transform matrix an affine object uses.
Since we, again, won't touch affine stuff until later, we can ignore that for now.

If an object is *not* in affine mode, you can apply a plain veritcal and/or horizontal flip to the object using bits 12 and 13.

The object's Size, when combined with the Orientation (see above) sets the object's width and height:

| Size | Square | Horizontal | Vertical |
|:-:|:-:|:-:|:-:|
| 0 | 8x8 | 16x8 | 8x16 |
| 1 | 16x16 | 32x8 | 8x32 |
| 2 | 32x32 | 32x16 | 16x32 |
| 3 | 64x64 | 64x32 | 32x64 |

### Object Attribute 2

| Bit(s) | Setting |
|:-:|:-|
| 0-9  | Base Tile ID (10 bits, `0..=1023`) |
| 10-11| Priority (lower is closer to the viewer) |
| 12-15| Palbank index (if 4bpp) |

The tile selection for objects can be a little strange at first.

* You pick a base tile index to start with. This determines the upper left tile of the object (the upper left 8x8).
* If the object is wider than 8 pixels, additional tiles at +1 index each are used to fill in the rest of the row.
* If the object is taller than 8 pixels, additional tiles are used to fill in the following rows.
  * When the Display Control is set for linear object tile mapping ("1d mapping"), each following row is +1 tile index from the index of the *last* tile in the previous row.
  * When the Display Control is *not* set for linear object tiles ("2d mapping"), each following row is +32 tile indexes from the *first* tile in the previous row.

That can be a little confusing at first, so here's a small diagram.

```
// offsets for 1d / linear tiles
 0   1   2   3
 4   5   6   7
 8   9  10  11
12  13  14  15

// offsets for 2d / non-linear 
 0   1   2   3
32  33  34  35
64  65  66  67
96  97  98  99
```

I think the origonal idea here was that if you imagine the object tile region to be specifically a 32 tile wide space of 32 rows, well then the 2d mapping mode *might* work well for you.
All the source "sprite sheet" art can be transformed into memory relatively "as is", and a 2x2 or 4x4 or whatever size object can just be picked out of a spot in the sprite sheet and the rest of the tiles will fill in properly.
I've never talked to a nintendo dev, but I think that was the idea.

Either way, I don't think that the 2d layout style is really for me.
I think that it's much easier to just use the 1d style and set up any art data apropriately during the compilation.

This is another one of those "any style can work" things.
The hardware doesn't care which one you pick, as long as all the settings line up you'll see a picture.

### Object Positioning

The X and Y coordinates of an object set the position of its *upper left* corner.
The Y axis increases *downward*, and the X axis increases *rightward*.
Object coordinates use wrapping integer addition both vertically and horizontally.
This means that it's usually most useful to think of coordinate values as being signed values, but using unsigned values will give the same result.
One minor problem is that Rust doesn't have a native `i9` datatype.
If we use an `i16` and mask the lowest 9 bits into the attribute bits then we'll get the right bits in the end.
It's a little bit awkward, but not really that bad.

## Showing Static Objects

## Moving The Objects Way Too Fast

## Waiting For Vertical Blank
