# GBA Overview

A lot of things to do with GBA programming in one area will often slightly touch on things from another area as well.

Before trying to cover any particular subject in close detail, let's get a "high level" summary of the overall abilities and limits of the device.

## User Input

There's a "plus pad" giving 4-direction input.
Also two main face buttons "A" and "B", along with two secondary face buttons "Start" and "Select".
Finally there's two shoulder buttons "L" and "R".
All these controls are digital, they read as either pressed or not-pressed.

The two main GBA models, the original and the "SP", have slightly different orientation of the controls, but they still both have the same set of controls.
The A button is always to the right of the B button.
Start and Select aren't so consistent: on the original model they're vertically oriented (Start on top), and with the SP they're horizontal (Start on the right).

## Video Capabilities

The GBA's screen is 240 pixels wide and 160 pixels tall.

The pixels support Red/Green/Blue color, with 5 bits per channel.

There's 4 background layers that can use various display modes.
The GBA can also display 128 "objects" on the screen, independent of the backgrounds.
Layers and objects have a "priority" system when there's overlap, which you can think of as being the "distance" from the viewer.
Lower priority elements are "closer" and so they will be drawn when they overlap with a higher priority element.
Objects always draw over top of a background of the same priority.
When two layers or two objects have the same priority, the lower index element gets drawn.

While one of the background modes does support direct-color "bitmap" output, it takes an immense amount of CPU time to animate a full bitmap image.
Almost all GBA games are drawn using one of the indirect-color "paletted" background modes.
Paletted color on the GBA can be in either 4-bits per pixel (4bpp) or 8-bits per pixel (8bpp).
An index value of 0 is always a "transparent" pixel, which shows the element "behind" the given pixel.
This means that 4-bit images have 15 actual colors available to them, and 8-bit images have 255 colors available.

While drawing, pixels are updated from left to right in each line, and top to bottom down the screen.
The display unit takes 4 CPU cycles to determine the color for each pixel.
After the visible 240 pixels of each line are drawn, there's also a 68-pixel "horizontal blank" (h-blank) before the next line begins.
After all 160 lines are drawn, there's also a 68-line "vertical blank" (v-blank) period before the screen draw cycle starts over.

Changes to any video related memory or controls usually take effect as soon as they are made.
Video settings should usually only be altered during h-blank and/or v-blank periods if you want to avoid graphical artifacts.

The entire draw loop runs at 59.73 FPS, which means that GBA games can run at "60 fps" if you round up just a tiny bit.

## Sound Capabilities

The GBA has the same four sound generator chips that were available on the original Game Boy:
* Two pulse generators.
* One programmable wave loop generator.
* One noise generator.

The GBA also has two First-in-first-out (FIFO) buffers which can play back 8-bit per sample audio recordings.

## Direct Memory Access (DMA)

There are four Direct Memory Access units in the GBA.
Basically these are specialized units for copying data between two parts of memory.
They can also be used to set a span of memory to a particular value.

The GBA's DMA units perform a "block transfer", sometimes called a "burst transfer".
This means that once they begin operating, the entire transfer is completed before they stop.
While they are in operation, the CPU's execution is paused.

The four different DMA unit each have some restrictions on what address regions they can use as source and destination address values.

A DMA memory copy will run slightly faster than even a well optimized CPU copying routine.
A DMA memory set will usually run slightly slower than an optimized CPU routine.

The main advantage of using DMA is that they can be set to perform a transfer automatically under certain conditions.
* At the start of h-blank
* At the start of v-blank
* When a FIFO sound buffer is empty.

The downside of using the DMA units is that, because they pause the CPU, they prevent hardware interrupts from being handled until after the transfer is complete.
This is not always a problem, but it's sometimes preferable to have the CPU do a "normal" memory copy if interrupt timing is essential in your program.

## Timers

There's four timer units in the GBA.

Each timer is a `u16` counter, and has a configurable "reload" value that the counter is set to.
When the timer ticks, the counter goes up by 1.
When the counter exceeds `u16::MAX` then the timer "overflows" and resets to the reload value.

A timer can be configured to tick every 1, 64, 256, or 1024 CPU cycles.
Alternatively, timer N can be configured to tick when timer N-1 overflows.
This second mode can only be used with timers 1, 2, and 3, since timer 0 doesn't have an "N-1" timer.

Between the configurable reset value and tick speed, you can configure the timers to tick at just about any speed you could ever need.

## Serial Port Communication

The GBA has a serial port, which mostly allows communication with other GBAs via link cable.

There's other hardware that can be hooked to the GBA's serial port,  but mostly it's for the link cable.

## The CPU's Address Space

| Address | Region |
|:-|:-|
| `0x0000_0000` | BIOS Code |
| `0x0200_0000` | External Work RAM (EWRAM) |
| `0x0300_0000` | Internal Work RAM (IWRAM) |
| `0x0400_0000` | IO Controls |
| `0x0500_0000` | Palette RAM (PALRAM) |
| `0x0600_0000` | Video RAM (VRAM) |
| `0x0700_0000` | Object Attribute Memory (OAM) |
| `0x0800_0000` | ROM Data |
| `0x0E00_0000` | Save RAM (SRAM) |

### BIOS Code

The Basic Input/Output System (BIOS) of the GBA is part of the device itself,
rather than being part of each game cart.

A special hardware lockout system prevents the BIOS data from being read normally except when the CPU's `pc` register is pointed at the BIOS.
This allows BIOS code to execute while providing a basic level of copy protection.
Unfortunately for the designers of the GBA, there are bugs within the BIOS code that allow BIOS information to be extracted.
Various "dumps" of the BIOS data (including the disassembled code) can be found, if you need to know something exact.
However, while hardware isn't covered by copyright, software code is.
Because of that, this book won't be diving into the BIOS line by line or anything like that.

The BIOS runs some code when:
* The GBA first boots (to animate the logo, and play the boot up sound).
  After the BIOS finishes this boot sequence it transfers control to the user's program.
  For "standard" programs with one game cart this is the start of ROM.
  For "multiboot" programs this is the start of EWRAM.
  This book does not plan to cover multiboot programs, we'll be sticking to the standard setup.
* A software interrupt is generated by the running program.
  The BIOS runs the software interrupt function signalled for and then returns.
  From the program's perspective using a software interrupt works "like a function call",
  except that there's about a 60 CPU cycle overhead just for the call itself.
  Normally a function call costs only about 6 cycles on the GBA.
* A hardware interrupt is generated by one of the GBA's components.
  The BIOS partly handles the hardware interrupt,
  then it calls a "user handler" to do any additional work,
  then the user handler does whatever and returns from that call (using a normal function return),
  then the BIOS handler also returns (but using the special "interrupt return" instruction that the CPU recognizes).
  This happens "in between" two instructions somewhere in the main program.
  A hardware interrupt is *similar to, but not actually the same as* having an additional thread that exists and sometimes does work.

### External Work RAM

The EWRAM memory is a 256k byte region with no default usage.

The EWRAM has 2 wait cycles per access.
This means that a single access (read or write) takes 3 CPU cycles total: wait, wait, then the access completes.

EWRAM also only has a 16-bit bus, which means that no more than 16 bits can be transferred per access.
Accessing an 8-bit or 16-bit value can happen all at once (in 3 CPU cycles),
but accessing a 32-bit value must be split into two smaller accesses (in 6 CPU cycles total).
You don't have to worry about the splitting part, the CPU and memory chip do that totally transparently.

Still, this makes EWRAM relatively slower than program stack variables (usually in IWRAM, see below).

### Internal Work RAM

The IWRAM is 32k bytes, and has some of the bytes already allocated.

* The top 256 bytes of IWRAM are reserved.
  They are taken up by BIOS usage, and for the very small stacks used by the alternate CPU modes.
* The stack of the system/user CPU mode is also in IWRAM.
  This is what our Rust program uses (generally the Rust program runs the CPU in system mode).
  The system stack starts at `0x0300_7F00`.
  When values are pushed onto the stack, the stack pointer address *decreases*.
* The rest of IWRAM is free for you to use as you like.
  The exact amount of "do what you want" space available depends on how much stack memory your program uses.
  Different programs use the stack different amounts, but it would be "slightly unusual" for a GBA program to take up more than 1k of stack.
  Unfortunately, there's no automated way for a stack overflow to be detected on the GBA.

IWRAM does not require any wait cycles, and has a 32-bit bus.
This means that IWRAM can be used at the full speed of the GBA's CPU (a whopping 16MHz).

It is possible to allocate both static variables and even function code into IWRAM.
The actual initialization data always lives in ROM,
but the linker can allocate values to have an IWRAM address,
then at the start of the program (before any Rust code executes) the appropriate data can be copied from ROM to RAM by assembly code.
Placing the "hot" code in IWRAM is very commonly done.

### IO Controls

To control all of the GBA's subsystems, Memory Mapped IO (MMIO) is used.
To make a given part of the device go into whatever specific state, a specific value is written to an appropriate address.
Similarly, to get information from a part of the GBA, you read from the appropriate address.

If you've never used MMIO before, it's a little magical seeming, but it's really nothing that spooky.
When the CPU reads and writes normal memory it's "just" sending a signal along the device's data bus to the memory chip.
Using MMIO is similarly "just" sending a signal to some other chip.

To control all the part of the GBA, we "just" have to learn what addresses expect what values to be written and/or read.
There's a lot of different parts, but most of them are each fairly simple, so slow and steady will get us though.

In this book we'll be using the [voladdress](https://docs.rs/voladdress) crate to make MMIO much more safe and easy to interact with, compared to just having raw pointers.

### Palette RAM

The palette memory has space for 256 background colors, followed immediately by 256 object colors.

When a visual element is in 8bpp mode, the background or object indexes directly into the full array, with the 8-bit value for each pixel.

When a visual element is in 4bpp mode it will also have a "palbank" value associated with it.
The palbank sets the upper 4 bits of the index, while the pixel value for the image selects the lower 4 bits.
In other words, the palbank selects which of the 16 sub-groupings the image uses, and the index values in the image select within the bank.

In both modes, an index value of 0 always makes that pixel transparent.
This means that 8bpp images can use up to 255 colors (plus transparent),
while 4bpp images can use up to 15 colors (plus transparent).
The index 0 entry of the background palette and object palette are thus not ever actually shown in any background or object.

When no visual element would be drawn to a particular pixel, index 0 of the background palette is what's shown.
This is called the "backdrop" color.

Index 0 of the object palette has no special alternative use.
It's just there for the consistency, so that both palettes are 256 elements.

### Video RAM

Video RAM is 64k, plus 16k, plus 16k.
This is a little different from simply having 96k of space because the display can be set into one of 6 different modes, and the active mode determines how the middle chip is used.

The video modes are just designated by their number: 0 through 5.

In display mode 0, 1, and 2 there's 64k for background data, and 32k for object data.

In display mode 3, 4, and 5 the middle chunk of memory flips from being part of the objects to being part of the background.
You get 80k for background data and 16k for objects.

Tiles are always 8x8 squares.
The amount of memory used depends on the bit depth of the tile: a 4bpp tile is 32 bytes, and an 8bpp tile is 64 bytes.

Backgrounds also need a "tile map" as well as the tile data.
The tile map says which tiles go at what position within the background.
The tiles and tile maps all have to share the same space for background data.

Multi-tile objects are not nearly as configurable.
There's a single global setting for which of the two modes multi-tile all objects should use.
Because of this, you don't need to reserve any object data space for tile maps at least.

### Object Attribute Memory

There's 128 entries of object attribute data.
Each attribute entry is 6 bytes.
Usually this is treated like a tuple struct of three `u16` values, with the fields just being called called 0, 1, and 2.

In *between* the object attribute entries are elements of the affine transform matrix data.
Each parameter is an `i16`, and it takes four of them to form a complete 2x2 transform matrix.

So the overall memory pattern looks sorta like this:

```
obj_0.field0
obj_0.field1
obj_0.field2
affine_0.a
obj_1.field0
obj_1.field1
obj_1.field2
affine_0.b
obj_2.field0
obj_2.field1
obj_2.field2
affine_0.c
obj_3.field0
obj_3.field1
obj_3.field2
affine_0.d
```

This whole thing is repeated 32 times, so in total there's 128 object entries and 32 affine transform entries.

The object attributes select various visual details about each object: the size, the base tile index, the tile bit depth, the palbank if it's a 4bpp object, etc.

The affine transform matrix data allows for rotation and scaling of visual elements.
When an element is drawn using affine display, it will have an affine index associated with it.
That index selects which of the 32 affine matrix transforms it's drawn with.

Because these two use cases aren't necessarily related, it can be slightly annoying that they're interspersed in memory.
However, it's not really that bad once you get used to it.

### ROM Data

This is where most of the program data will be.
It's Read-Only Memory, but you get a whole heck of a lot of it.
Way more than usual for "embedded device" type hardware.
The GBA supports ROMs up to 32 megabytes.
Most commercial games stayed well under 16 megabytes, so you should absolutely be able to stay within the 32 megabyte limit.

The ROM has a 16-bit bus.
There's also some wait cycles as well, like with EWRAM.
The number of wait cycles depends on if the access is a "sequential" or "non-sequential" access (sequential accesses are faster).

There's actually two other mirrors for accessing ROM data.
In the table above I list ROM as `0x08...`, but there's also a mirror also at `0x0A...` and `0x0C...` as well.
This mostly only matters if you've got a special game pak where different memory is accessible at different wait speeds.

For the purposes of this guide, I'm just going to act like ROM is always at `0x08...` addresses.

### Save RAM

The Save RAM available depends on the cart being used. Usually up to 32k of save ram is available for the program to use.

Unfortunately, the SRAM has some strong limitations:

* There's only an 8-bit bus, and larger accesses aren't automatically split up like with other memory regions.
  We have to use special copy routines to get data in and out of SRAM.
* You can't access SRAM and ROM in the same instruction, so for the program to run properly the special copying code has to be placed in RAM.

It's stuff that we can cope with, but it's still a little annoying.
