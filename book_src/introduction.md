# Introduction

This is a series about how to program for the Game Boy Advance (GBA) using the Rust programming language.

You'll need to use a Nightly toolchain for GBA development,
specifically we'll need the unstable [build-std](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) feature of `cargo`.

## Non-`cargo` Tools

Every Rust programmer expects to use `cargo` for building a Rust project,
but we'll also want some extra tools.

### A GBA Emulator

It's possible to run the programs we'll be building on actual hardware.
However, during development it's generally much easier to just run the program in an emulator.

A "good" emulator for our purposes is not only accurate, but also has features to help our development.
Things like the ability to check the current state of all hardware settings, look at memory, debug the program, and so on.

I use [mgba](https://mgba.io/), because it can run the ELF output from compiling with Rust directly, without having to convert the data into a ROM image.
This makes it compatible with `cargo run --example abc` and similar cargo commands.
I also know that some on the GBA Development Discord use [no$gba](https://www.nogba.com/), though I've never used it myself.

### A Set Of Binutils

The "binutils" for a toolchain are utilities to work on binaries that the toolchain produces.
If we want to convert our compiler output into a usable ROM image (or do other neat things), we'll need to use the binary utils.

Since there's two major open source C toolchains, there's two main sets of binutils:
one for LLVM and one for GNU.

* The [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) extension for cargo lets us access the binutils for the LLVM that comes with our Rust toolchain.
  Once you have this, all the "usual" binutils become available as cargo subcommands.
  Running `objdump` would be `cargo objdump`
* Alternately, you can install the [ARM GNU Binutils](https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain).
  In this case, you'll want to also [configure cargo](https://doc.rust-lang.org/cargo/reference/config.html#configuration-format) to use the GNU Binutils linker.
  That way, the debug info nd such in your executables will be compatible with what the other GNU binutils expect.
  When using the GNU binutils, each utility is a separate program, and for non-host targets the target name prefixes the util name.
  To run `objdump` on our GBA executable we'd run `arm-none-eabi-objdump`.
  You'd also need to pass the correct path to your executable files, which are generally buried in the `target/` directory.

### `cargo-show-asm`

[GitHub](https://github.com/pacak/cargo-show-asm)

Using `objdump`, you can disassemble an executable file back into assembly code.
This is very neat, and very useful, but sometimes `cargo-show-asm` is *sometimes better*.

What `cargo-show-asm` does is let you check the mir, llvm-ir, or even the assembly that's sent to the assembler.
This can be more readable than looking at disassembler output.

### `gbafix`

[GitHub](https://github.com/rust-console/gbafix)

When we compile a ROM, we'll end up with a blank ROM header.
This is fine if we only want to run the program in an emulator, but won't work on real hardware.
Running the `gbafix` program can patch the header so that the ROM is suitable for use with hardware.

There's a C version that works just fine, but since it's a very small program I wrote a Rust version as well which can be installed through `cargo`:

```
cargo install gbafix
```

### `grit`

[Original Website](https://www.coranac.com/projects/grit/) (with pre-built Win32 binaries)

[Github](https://github.com/devkitPro/grit) (C source only)

This is a tool that converts image files into a format suitable on the GBA.
It outputs the results as either assembly code or C code, containing a list of `u32` constants.
Just copy this list of values into your Rust code as a `&[u32]` slice and you'll have the image data you need.
The palette data will be separate from the image data, you'll probably want both.

## External Reference Materials

This guide will definitely not explain every possible thing that you'd want to know about the GBA.
If you need to know more, try one of these.

* [gbatek](https://problemkaputt.de/gbatek.htm) (html)
  This is the standard homebrew reference for all things GBA / DS / DSi related.
  It contains just about everything you'd need to know about the MMIO control for the GBA.
  The only downside is that the notes are sometimes a little cryptic.
* [ARM Architecture Reference Manual](https://www.intel.com/content/dam/www/programmable/us/en/pdfs/literature/third-party/archives/ddi0100e_arm_arm.pdf) (pdf)
  This version is published in 2000, and so covers the ARMv5T architecture.
  The GBA uses the ARMv4T architecture, which means it's covered by this document.
  Just ignore any details that they were added for v5T.
* [ARM7TDMI Reference Manual](https://documentation-service.arm.com/static/5e8e1323fd977155116a3129?token=) (pdf)
  Within the ARMv4T architecture, the GBA's exact CPU is an ARM7TDMI.
  This document can be relevant if you need close CPU details.
* [The Awesome GBA Dev Repository](https://github.com/gbadev-org/awesome-gbadev) (github)
  This has all sorts of good GBA development information.
  Particularly, there's links to a forum and a discord so that you can chat with the community and hopefully get your questions answered.

# Project License

The work in this project is licensed as follows:

* Rust code: `Zlib OR Apache-2.0 OR MIT`
* All other content (linker scripts, non-Rust book content, etc): `CC0-1.0`

# Supporting The Project

If you'd like to support the book you can sign up to be a [Github Sponsor](https://github.com/sponsors/Lokathor).
