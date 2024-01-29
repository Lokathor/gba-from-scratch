# Introduction

This is a series about how to program for the Game Boy Advance (GBA) using the Rust programming language.

# GBA Overview

A lot of things to do with GBA programming in one area will often slightly touch on things from another area as well.

Before trying to cover any particular subject in close detail, let's get a "high level" summary of the overall abilities and limits of the device.

## Address Space

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

## Video

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
Paletted color on the GBA can be in either 4-bits per pixel or 8-bits per pixel.
An index value of 0 is always a "transparent" pixel, which shows the element "behind" the given pixel.
This means that 4-bit images have 15 actual colors available to them, and 8-bit images have 255 colors available.

While drawing, pixels are updated from left to right in each line, and top to bottom down the screen.
The display unit takes 4 CPU cycles to determine the color for each pixel.
After the visible 240 pixels of each line are drawn, there's also a 68-pixel "horizontal blank" before the next line begins.
After the all 160 lines are drawn, there's also a 68-line "vertical blank" period before the screen draw cycle starts over.
The entire draw loop runs at 59.73 FPS, which means that GBA games can run at "60 fps" if you round up just a tiny bit.

## Sound

## Interrupt Requests (IRQ)

## Direct Memory Access (DMA)

## Timers

# Non-`cargo` Tools

## gbafix

[GitHub](https://github.com/rust-console/gbafix)

## grit

[Original Website](https://www.coranac.com/projects/grit/) (with pre-built Win32 binaries)

[Github](https://github.com/devkitPro/grit) (C source only)

# Rust Project Outline

## Cargo Configuration

## Linker Script

## `_start` fn

## Assembly Interrupt Handler

## `main` fn

# External Reference Materials
* [gbatek](https://problemkaputt.de/gbatek.htm) (html)
  This is the standard homebrew reference for all things GBA / DS / DSi related.
* [ARM Architecture Reference Manual](https://www.intel.com/content/dam/www/programmable/us/en/pdfs/literature/third-party/archives/ddi0100e_arm_arm.pdf) (pdf)
  This version is published in 2000, and so covers the ARMv5T architecture.
  The GBA uses the ARMv4T architecture, which means it's covered by this document.
  Just ignore any details that they were added for v5T.
* [ARM7TDMI Reference Manual](https://documentation-service.arm.com/static/5e8e1323fd977155116a3129?token=) (pdf)
  Within the ARMv4T architecture, the GBA's exact CPU is an ARM7TDMI.
  This document can be relevent if you need close CPU details.

# Project License

The work in this project is licensed as follows:

* Rust code: `Zlib OR Apache-2.0 OR MIT`
* All other content (linker scripts, book text, etc): `CC0-1.0`

# Supporting The Project

If you'd like to support the book you can sign up to be a [Github Sponsor](https://github.com/sponsors/Lokathor).
