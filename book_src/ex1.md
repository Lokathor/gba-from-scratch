
# Basics

Let's program some stuff to run on the GBA.

## Basic Compilation

As usual with any new Rust project we'll need a `Cargo.toml` file:

```toml
# Cargo.toml

[package]
name = "gba_from_scratch"
version = "0.1.0"
edition = "2021"
```

And we want some sort of program to run so let's make an example called `ex1.rs` in the `examples/` directory.
It can just be a classic "Hello, World" type program to start.

```rust
// examples/ex1.rs

fn main() {
  println!("hello");
}
```

Since we're not running the compiler on the GBA itself, then we'll need to "cross-compile" our program.
It's called "cross compilation" when you build a program for some system *other* than the system that you're running the compiler on.
The system running the compiler is called the "host" system, and the system you're building for is called the "target" system.
In our case, the host system can be basically anything that can run a Rust toolchain.
I've had success on Windows, Linux, and Mac, there's no big difficulties.

To do a cross compile, we pass [--target](https://doc.rust-lang.org/cargo/commands/cargo-build.html#compilation-options) to `cargo`.
If we look up the [Game Boy Advance](https://en.wikipedia.org/wiki/Game_Boy_Advance) on wikipedia, we can see that it has an [ARM7TDMI](https://en.wikipedia.org/wiki/ARM7#ARM7TDMI) CPU.
The "ARM7T" part means that it uses the "ARMv4T" CPU architecture.
Now we go the [Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html) page and use "ctrl+F" to look for "ARMv4T".
We can see three(-ish) entries that might(?) be what we want.

* `armv4t-none-eabi`
* `armv4t-unknown-linux-gnueabi`
* `thumbv4t-none-eabi`

This is the part where my "teach like you're telling a story" style breaks down a bit.
What should happen next is that we pick the `thumbv4t-none-eabi` target.
Except there's not an easy to find document that tells you this step that I can just link to and have you read a few lines.
The shortest version of the full explanation is something like "Many ARM CPUs support two code 'states', and one of them is called '[thumb](https://en.wikipedia.org/wiki/ARM_architecture_family#Thumb)', and that's the better default on the GBA."
We can certainly talk more about that later, but for now you just gotta go with it.

Let's see what happens when we pass `--target thumbv4t-none-eabi` as part of a call to `cargo`:

```
>cargo build --example ex1 --target thumbv4t-none-eabi
   Compiling gba_from_scratch v0.1.0 (D:\dev\gba-from-scratch)
error[E0463]: can't find crate for `std`
  |
  = note: the `thumbv4t-none-eabi` target may not be installed
  = help: consider downloading the target with `rustup target add thumbv4t-none-eabi`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`

error: requires `sized` lang_item

For more information about this error, try `rustc --explain E0463`.
error: could not compile `gba_from_scratch` (lib) due to 2 previous errors
```

Well we seem to have already configured something wrong, somehow.
The trouble with a wrong project configuration is that the compiler can't always guess what you *meant* to do.
This means that the error message suggestions might be helpful, but they also might lead you down the wrong path.

One suggested way to fix the problem is to add the `thumbv4t-none-eabi` target with `rustup`.
It seems pretty low risk to just try installing that, so let's see.

```
>rustup target add thumbv4t-none-eabi
error: toolchain 'nightly-x86_64-pc-windows-msvc' does not contain component 'rust-std' for target 'thumbv4t-none-eabi'; did you mean 'thumbv6m-none-eabi'?
note: not all platforms have the standard library pre-compiled: https://doc.rust-lang.org/nightly/rustc/platform-support.html
help: consider using `cargo build -Z build-std` instead
```

Ah, dang.
If we double check the Platform Support page we might see that `thumbv4t-none-eabi` is in the "Tier 3" section.
Tier 3 targets don't have a standard library available in `rustup`.

How about this `build-std` thing?
The `-Z` flags are all unstable flags, so we can check the [unstable section](https://doc.rust-lang.org/cargo/reference/unstable.html) of the cargo manual.
Looks like [build-std](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) lets us build our own standard library.
We're going to need Nightly rust, so set that up how you want if you need to.
You can use `rustup default nightly` (which sets the *system global* default), or you can use a [toolchain file](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file) if you want to use Nightly on just this one project.
Once we've set for Nightly use, we need to get the `rust-src` component from `rustup` too.

```
rustup default nightly
rustup component add rust-src
```

Okay let's try again

```
> cargo build --example ex1 --target thumbv4t-none-eabi -Z build-std
   Compiling compiler_builtins v0.1.89
   Compiling core v0.0.0 (/Users/dg/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core)
   Compiling libc v0.2.140
   Compiling cc v1.0.77
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/Users/dg/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std)
   Compiling unwind v0.0.0 (/Users/dg/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/Users/dg/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/Users/dg/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/alloc)
   Compiling cfg-if v1.0.0
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
   Compiling rustc-std-workspace-alloc v1.99.0 (/Users/dg/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/rustc-std-workspace-alloc)
   Compiling panic_abort v0.0.0 (/Users/dg/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/panic_abort)
   Compiling panic_unwind v0.0.0 (/Users/dg/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/panic_unwind)
   Compiling gimli v0.26.2
   Compiling miniz_oxide v0.5.3
   Compiling hashbrown v0.12.3
   Compiling object v0.29.0
   Compiling std_detect v0.1.5 (/Users/dg/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/stdarch/crates/std_detect)
error[E0432]: unresolved import `alloc::sync`
 --> /Users/dg/.cargo/registry/src/index.crates.io-6f17d22bba15001f/gimli-0.26.2/src/read/dwarf.rs:2:12
  |
2 | use alloc::sync::Arc;
  |            ^^^^ could not find `sync` in `alloc`

For more information about this error, try `rustc --explain E0432`.
error: could not compile `gimli` (lib) due to previous error
warning: build failed, waiting for other jobs to finish...
```

Whoa... that's way too much.
We didn't mean for all of that to happen.
Let's check that cargo manual again.
Ah, it says we need to pass an argument to our command line argument if we don't want as much stuff to be build

```
> cargo build --example ex1 --target thumbv4t-none-eabi -Z build-std=core 
   Compiling gba_from_scratch v0.1.0 (/Users/dg/gba-from-scratch)
error[E0463]: can't find crate for `std`
  |
  = note: the `thumbv4t-none-eabi` target may not support the standard library
  = note: `std` is required by `gba_from_scratch` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`

For more information about this error, try `rustc --explain E0463`.
error: could not compile `gba_from_scratch` (lib) due to previous error
```

That's different from before at least.
Well, we told to to only build `core` and not `std`, and then it said we couldn't use `std`.
Makes sense.
Lets change the example.

```rs
// ex1.rs
#![no_std]

fn main() {
  println!("hello");
}
```

And we need to fix our `lib.rs` to also be `no_std`.
It doesn't do anything else for now, it's just blank beyond being no_std.

```rust
// lib.rs
#![no_std]
```

Now rust-analyzer is telling me we can't use println in our example.
Also, we're missing a `#[panic_handler]`.
Here's the error.

```
> cargo build --example ex1 --target thumbv4t-none-eabi -Z build-std=core
   Compiling gba_from_scratch v0.1.0 (/Users/dg/gba-from-scratch)
error: cannot find macro `println` in this scope
 --> examples/ex1.rs:4:3
  |
4 |   println!("hello");
  |   ^^^^^^^

error: `#[panic_handler]` function required, but not found

error: could not compile `gba_from_scratch` (example "ex1") due to 2 previous errors
```

Well, we can comment out the `println!`.
For the panic handler, we go to the [Attributes](https://doc.rust-lang.org/reference/attributes.html) part of the rust reference.
That links us to [panic_handler](https://doc.rust-lang.org/reference/runtime.html#the-panic_handler-attribute), which sets what function gets called in event of panic.

```rust
// ex1.rs
#![no_std]

fn main() {
  //
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
```

Now we get a new, *different* error when we try to build:

```
> cargo build --example ex1 --target thumbv4t-none-eabi -Z build-std=core
   Compiling gba_from_scratch v0.1.0 (/Users/dg/gba-from-scratch)
error: requires `start` lang_item

error: could not compile `gba_from_scratch` (example "ex1") due to previous error
```

Alright so what's this `start` lang item deal?
Well it has to do with the operating system being able to run your executable.
The details aren't important for us, because there's no operating system on the GBA.
Instead of trying to work with the `start` thing, we'll declare our program as `#![no_main]`.
This prevents the compiler from automatically generating the `main` entry fn, which is what's looking to call that start fn.
Note that this generated `main` fn is *separate* from the `main` fn that we normally think of as being the start of the program.
Because, as always, programmers are very good at naming things.

```rust
// ex1.rs
#![no_std]
#![no_main]

fn main() {
  //
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
```

Okay let's try another build.

```
> cargo build --example ex1 --target thumbv4t-none-eabi -Z build-std=core
   Compiling gba_from_scratch v0.1.0 (/Users/dg/gba-from-scratch)
warning: function `main` is never used
 --> examples/ex1.rs:4:4
  |
4 | fn main() {
  |    ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `gba_from_scratch` (example "ex1") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.64s
```

Okay.
It builds.

Let's see if it works I guess.
Personally I like to use [mGBA](https://mgba.io/) as my emulator of choice, but any GBA emulator should be fine.
If you're on Windows then your executable will be called `mgba.exe` by default, and if you're on Mac or Linux you'll get both `mgba` (no UI) and `mgba-qt` (has a menu bar and such around the video frame).
On my Windows machine I just made a copy of `mgba.exe` that's called `mgba-qt.exe` so that both names work on all of my devices.

```
> mgba target/thumbv4t-none-eabi/debug/examples/ex1
```

The emulator starts and then... shows a dialog box.
"An error occurred." says the box's title bar.
"Could not load game. Are you sure it's in the correct format?"
Well, sorry mgba, but we're not sure it's in the correct format.
In fact, we're pretty sure it's *not* the correct format right now.
I guess we'll have to inspect the compilation output.

## ARM Binutils

If we go to ARM's developer website we can fine the [ARM Toolchain Downloads](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads) page.
This lets us download the tools for working with executables for the `arm-none-eabi` family of targets.
This includes our `thumbv4t` program, as well as other variants of ARM code.
You can get it from their website, or if you're on a Linux you can probably get it from your package manager.

The binutils package for a target family has many individual tools.
The ones we'll be using will all be named `arm-none-eabi-` to start, to distinguish them from the same tool for other targets.
So if we want to use "objdump" we call it with `arm-none-eabi-objdump` and so on.
That's exactly what we want to use right now.
We pass the name of the compiled executable, and then whichever other options we want.
For now let's look at the `--section-headers`

```
> arm-none-eabi-objdump target/thumbv4t-none-eabi/debug/examples/ex1 --section-headers

target/thumbv4t-none-eabi/debug/examples/ex1:     file format elf32-littlearm

Sections:
Idx Name          Size      VMA       LMA       File off  Algn
  0 .debug_abbrev 000000f4  00000000  00000000  00000094  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  1 .debug_info   000005a6  00000000  00000000  00000188  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  2 .debug_aranges 00000020  00000000  00000000  0000072e  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  3 .debug_str    00000495  00000000  00000000  0000074e  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  4 .debug_pubnames 000000c0  00000000  00000000  00000be3  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  5 .debug_pubtypes 00000364  00000000  00000000  00000ca3  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  6 .ARM.attributes 00000030  00000000  00000000  00001007  2**0
                  CONTENTS, READONLY
  7 .debug_frame  00000028  00000000  00000000  00001038  2**2
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  8 .debug_line   00000042  00000000  00000000  00001060  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  9 .comment      00000013  00000000  00000000  000010a2  2**0
                  CONTENTS, READONLY
```

There's a few columns of note:

* `Size` is the number of bytes for the section.
* `VMA` is the Virtual Memory Address. On the GBA this means the intended address when the main program is running. All of our data starts in ROM, and some of it we will copy into RAM just after boot. When a section is intended to be copied into RAM, it will have a VMA separate from the LMA.
* `LMA` is the Logical Memory Address. On the GBA this means the address in ROM.

Which means... according to the chart... none of this data would end up in the ROM?
I guess that means that, if we extracted our raw program from the [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) container file that the compiler uses, we would end up with a totally blank ROM.
That certainly doesn't sound like what mgba would call the "correct format".

## Linker Scripts

What's wrong is that we need to adjust the [linker script](https://sourceware.org/binutils/docs/ld/Scripts.html).
That link goes to the documentation for the binutils linker (called `ld`), and technically we're actually using the linker that ships with the compiler (called `rust-lld`).
`rust-lld` is the Rust version of `lld`, which is LLVM's linker that's intended to be a "drop in" replacement for GNU's `ld`.
Both linkers use a linker script system, and they both even use the same linker script format.
I tried to find an in depth manual for `lld` specifically, but all I could find was the top level "man page" explanations.
Referring to the the GNU `ld` manual will have to do.

You don't have to read the whole manual, the short story goes like this: linkers take one or more "object" files and "link" them into a single "executable" file.
The linker script is what guides the linker in exactly what to do.
If you don't say what script to use then the linker will use a default linker script that it keeps wherever.
When the target is a "normal" target like Windows or Mac then using a default linker script is just fine.
When the target is something a little more esoteric, like most embedded devices, including the GBA, then the default won't be good enough.
We'll have to write our own script and make the linker use that.

One complexity here is that the linker script to use is an argument passed to the linker.
And the way you pass args to the linker is that you tell `rustc` to do it.
Except with `cargo build` there's no way to tell `rustc` an extra argument.
We could use `cargo rustc`, but it's a pain to have to remember an alternate command.
As much as possible we'd like `cargo build` to work.
We could use a `build.rs` file to pass an arg to the linker, but making a build script just to pass one argument seems like maybe overkill.
Probably we should just set it as part of our the `RUSTFLAGS` environment variable.
The catch with `RUSTFLAGS` is that any time you change it you have to build *the entire crate graph* again.
We want to "write it down" (so to speak) and have it automatically be the same every time.
This can be done with a [cargo configuration](https://doc.rust-lang.org/cargo/reference/config.html#configuration) file.

First let's make a blank `normal_boot.ld` file in a `linker_scripts/` folder.
Then in the `.cargo` folder we fill in `config.toml`

```toml
# .cargo/config.toml

[target.thumbv4t-none-eabi]
rustflags = ["-Clink-arg=-Tlinker_scripts/normal_boot.ld"]
```

while we're at it, we can even set a default target (which is used when we don't specify `--target`, and we can configure for `build-std` to be automatically be used, all in the same file.

```toml
# .cargo/config.toml

[unstable]
build-std = ["core"]

[build]
target = "thumbv4t-none-eabi"

[target.thumbv4t-none-eabi]
rustflags = ["-Clink-arg=-Tlinker_scripts/normal_boot.ld"]
```

Great, let's try it out

```
> cargo build --example ex1
warning: function `main` is never used
 --> examples\ex1.rs:4:4
  |
4 | fn main() {
  |    ^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `gba_from_scratch` (example "ex1") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
```

Cool.
It's a lot less to type, and we're ready to fill in our linker script.

Our linker script is called `normal_boot.ld` because there's two ways for the GBA to boot up.
One of them is the "normal" style with a program running off of the game pak.
The other is "multiboot" where the GBA can download a program over the link cable.
Since we might want to do multiboot some day, we might as well give our linker script a specific name to start with.
Once things are set up we won't really have to think about it on a regular basis, so it's fine.

There's three things we'll have to concern ourselves with:

* The [entry point](https://sourceware.org/binutils/docs/ld/Entry-Point.html)
* The [memory](https://sourceware.org/binutils/docs/ld/MEMORY.html) locations
* The [sections](https://sourceware.org/binutils/docs/ld/SECTIONS.html)

Picking an entry point is easy, it's just the name of a symbol.
The traditional entry point name is just `_start`, so we'll go with that.

```ld
ENTRY(_start)
```

Having an entry point set *doesn't really matter* for running the program on actual GBA hardware.
Still when the entry point ends up at one of the usual address values, it helps the heuristic system mgba uses to determine if it should run our program as a normal game or a multiboot game, so it's not entirely useless.

Which brings us to the memory portion.

The GBA has three main chunks of memory: Read-Only Memory (ROM), Internal Work RAM (IWRAM), and External Work RAM (EWRAM).
We can cover more of the fine differences later, for now it's enough to write them down into our linker script.
For each one we have to specify the base address and the size in bytes.

```ld
MEMORY {
  ewram (w!x) : ORIGIN = 0x2000000, LENGTH = 256K
  iwram (w!x) : ORIGIN = 0x3000000, LENGTH = 32K
  rom (rx)    : ORIGIN = 0x8000000, LENGTH = 32M
}
```

Finally, we have to tell the linker which *output* section to assign all of the *input* sections it finds.
This uses a glob-matching sort of system.
We specify an output section that we want to have created, and then in the braces for it we list matchers that are checked against each input section the linker sees.
When an input section fits one of the matchers, it goes with that output section.

Program code is supposed to end up in the `.text` section, so we can start with just that.

```ld
SECTIONS {
  .text : {
    *(.text .text.*);
  } >rom
}
```

Here we've got one matcher listed, `*(.text .text.*);`.
The `*` at the start means it applies to any input file.
We could limit what files it applies to, if we wanted, but generally we shouldn't.
Inside the parenthesis is a space separated list of globs.
We've got two: `.text` and `.text.*`.
The first is for the exact match `.text`, and the second is for anything that starts with `.text.`.
The convention for section names is to start with a `.`, and they can't have spaces.
Rust will default to having every function in its own section, all with the prefix `.text.`.
Unused code can only be removed one entire input section at a time, so having every function in a distinct input section keeps our output as small as possible.

The `>rom` part after tha braces allocates the entire output section into the `rom` memory that we declared before.

All together, we've got this:

```ld
/* normal_boot.ld */
/* THIS LINKER SCRIPT FILE IS RELEASED TO THE PUBLIC DOMAIN (SPDX: CC0-1.0) */

ENTRY(_start)

MEMORY {
  ewram (w!x) : ORIGIN = 0x2000000, LENGTH = 256K
  iwram (w!x) : ORIGIN = 0x3000000, LENGTH = 32K
  rom (rx)    : ORIGIN = 0x8000000, LENGTH = 32M
}

SECTIONS {
  .text : {
    *(.text._start);
    *(.text .text.*);
  } >rom
}
```

This isn't a complete and "final" linker script, but for now it's enough to let us proceed.

If we rebuild the program right now we still won't get anything in the output `.text` section.
Remember that dead code warning we keep getting on our `main` function?
Nothing in our program ever calls `main`, and it's not public for outsiders to call, so it gets discarded during linking.
Since no code can call `main` then no code can panic either, and the `panic_handler` function gets removed as well.
We end up with nothing at all.

We need to add some code to our progam so that there will be something to output.
Might as well define the `_start` function.

`_start` doesn't work like a normal function.
The way the very start of the GBA's ROM works is special.
When the GBA first boots the BIOS (which is part of the GBA itself, not part of our ROM) takes control.
It and plays the boot animation and sound that you're probably familiar with, then does a checksum on our ROM's header data.
If the checksum passes the BIOS jumps control to `0x0800_0000` (the start of ROM).
That's where our `_start` will be.
The first instruction can be "anything" but immediateley after that is the rest of the header data.
That means that in practice the very first instruction of `_start` has to be a jump *past* the rest of the header data, since the header data isn't executable code.

Sticking non-executable data into the middle of a function isn't something that the compiler is really capable of dealing with, so we'll have to take direct control of the situation.
We could do this using either [global_assembly!](https://doc.rust-lang.org/core/arch/macro.global_asm.html) or a [#[naked]](https://github.com/rust-lang/rust/issues/90957) function.
One might think that we should pick the Stable option (global assembly), over the Nightly option (a naked function).
However, naked functions are basically much easier to work with.
Since using `build-std` means that we have to use Nightly anyway, it's not that bad to also use naked functions as well.
If naked functions were the very last thing that required us to use Nightly we could move to global assembly instead.

At the top of `ex1.rs` we need to add `#![feature(naked_functions)]`.

Then we add our `_start` function.
In addition to marking it as `#[naked]`, we also mark it `#[no_mangle]`.
We need to use `#[instruction_set(arm::a32)]` as well.
This is part of that arm/thumb thing from before.
Because the BIOS jumps to the start of the ROM with the CPU in a32 mode, our function must be encoded appropriately.
Since `_start` has got to specifically at the very start of the ROM we'll use `#[link_section = ".text._start"]` to assign our function a specific section name we can use in our linker script.
Since `_start` is going to be "called" by the outside world we have to assign it the `extern "C"` ABI.
Since it should never return we will mark the return type as `-> !`.
So far it all looks like this:

```rust
// ex1.rs

#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".text._start"]
unsafe extern "C" fn _start() -> ! {
  todo!()
}
```

Inside of the `_start` function, because it's a naked function, we must put an `asm!` block as the only statement.
Our assembly will be very simple for now.
Let's look at it on its own.

```arm
b 1f
.space 0xE0
1:
b 1b
```

In the first line we branch (`b`) to the label `1` that is "forward" from the instruction (`1f`).

Then with `.space` we put 0xE0 blank bytes.
This is called a "directive", it doesn't emit an instruction directly, instead it tells the assembler to do a special action.
We can tell it's a directive because it has a `.` at the beginning.
The blank space is where the header data can go when we need to fill it in.
mgba doesn't check the header, so during development it's fine to leave the header blank.
We can always fix the header data after compilation using a special tool called `gbafix` when we need to.

The `1:` is a label.
We know it's a label because it ends with `:`.
Unlike with function names, a label can be just a number.
In fact, it's *preferred* to only use numberic labels whenever possible.
When a non-numeric label is defined more than once it causes problems (that's why function names are mangled by default, and we had to use `no_mangle`).
When a numeric label is defined more than once, all instances of that label can co-exist just fine.
When you jump to a numbered label (forward or back), it just jumps to the closest instance of that number (in whichever direction).
Note that a label *can* have something else on the same line following the `:`.
Usually a label will be on a line of its own so that it stands out a little more in the code, but that's just a code style thing.
Something can follow a label on the same line as well.
If a label is on a line of its own, the label "points to" the next line that has a non-label thing on it.
You can also have more than one label point at the same line, if necessary.

Finally, our second actual instruction is that we want to branch backward to the label `1`.
Since that `1` label points at the branch itself, this instruction causes an infinite loop.
The same as if we'd written `loop {}` in rust.

At the end of our assembly we have to put `options(noreturn)`.
That's just part of how `#[naked]` functions work.
So when we put it all together we get this:

```rust
// ex1.rs

#[naked]
#[no_mangle]
#[link_section = ".text._start"]
unsafe extern "C" fn _start() -> ! {
  core::arch::asm! {
    "b 1f",
    ".space 0xE0",
    "1:",
    "b 1b",
    options(noreturn)
  }
}
```

And we also want to adjust the linker script.
Since `_start` is now in `.text._start`, we'll put a special matcher for that to make sure it stays at the start of the ROM, no matter what order the linker sees our files in.

```ld
/* normal_boot.ld */

SECTIONS {
  .text : {
    *(.text._start);
    *(.text .text.*);
  } >rom
}
```

And after all of this, we can build our example and see that something shows up in the `.text` section of the executable.

```
> cargo build --example ex1 && arm-none-eabi-objdump target/thumbv4t-none-eabi/debug/examples/ex1 --section-headers
   Compiling core v0.0.0 (C:\Users\Daniel\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core)
   Compiling rustc-std-workspace-core v1.99.0 (C:\Users\Daniel\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\rustc-std-workspace-core)
   Compiling compiler_builtins v0.1.89
   Compiling gba_from_scratch v0.1.0 (D:\dev\gba-from-scratch)
    Finished dev [unoptimized + debuginfo] target(s) in 9.98s

target/thumbv4t-none-eabi/debug/examples/ex1:     file format elf32-littlearm

Sections:
Idx Name          Size      VMA       LMA       File off  Algn
  0 .text         000000e6  08000000  08000000  00010000  2**1
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  1 .ARM.exidx    00000010  080000e8  080000e8  000100e8  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  2 .debug_abbrev 0000010a  00000000  00000000  000100f8  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  3 .debug_info   000005b7  00000000  00000000  00010202  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  4 .debug_aranges 00000028  00000000  00000000  000107b9  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  5 .debug_ranges 00000018  00000000  00000000  000107e1  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  6 .debug_str    0000049c  00000000  00000000  000107f9  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  7 .debug_pubnames 000000cb  00000000  00000000  00010c95  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  8 .debug_pubtypes 00000364  00000000  00000000  00010d60  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
  9 .ARM.attributes 00000030  00000000  00000000  000110c4  2**0
                  CONTENTS, READONLY
 10 .debug_frame  00000038  00000000  00000000  000110f4  2**2
                  CONTENTS, READONLY, DEBUGGING, OCTETS
 11 .debug_line   00000056  00000000  00000000  0001112c  2**0
                  CONTENTS, READONLY, DEBUGGING, OCTETS
 12 .comment      00000013  00000000  00000000  00011182  2**0
                  CONTENTS, READONLY
```

I think we're ready to test the program.
Obviously we just use `cargo run` and...

```
> cargo run --example ex1
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target\thumbv4t-none-eabi\debug\examples\ex1`
error: could not execute process `target\thumbv4t-none-eabi\debug\examples\ex1` (never executed)

Caused by:
  %1 is not a valid Win32 application. (os error 193)
```

Ah, right, Windows doesn't know how to run GBA programs, of course.

Instead, let's adjust the `.cargo/config.toml` to set a "runner" value in our target confituration.
When we have a runner set, `cargo run` will call the runner program and pass the program we picked as the first argument.

```toml
# .cargo/config.toml 

[target.thumbv4t-none-eabi]
rustflags = ["-Clink-arg=-Tlinker_scripts/normal_boot.ld"]
runner = "mgba-qt" #remove the -qt part if you're on Windows!
```

And so we try again

```
> cargo run --example ex1
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `mgba-qt target\thumbv4t-none-eabi\debug\examples\ex1`
```

If everything is right so far, mGBA should launch and show a white screen.
Congrats, it didn't crash.

If we want to double check that our code is showing up in the executable properly we can even use `objdump` to check that.
If we pass `--disassemble` we can get a printout of the assembly.
There's a bunch of other options for how to configure that output too, so check the `--help` output to see what you can do.
I like to use `--demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std`, and you get output like this:

```
> arm-none-eabi-objdump target/thumbv4t-none-eabi/debug/examples/ex1 --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std

target/thumbv4t-none-eabi/debug/examples/ex1:     file format elf32-littlearm


Disassembly of section .text:

08000000 <_start>:
 8000000:       b       80000e4 <_start+0xe4>
        ...
 80000e4:       b       80000e4 <_start+0xe4>
 80000e8:       udf     #65006  ; 0xfdee
```

Disassembly is a tricky thing sometimes.
It's not always clear to the disassembler what is code and what's data.
Or when it should decode `a32` code (4 bytes each) or `t32` code (2 bytes each).
In this case, the disassembler did notice that enough bytes in a row are all zero, and it just cuts that from the output with a `...`.
That's cool, but it doesn't *always* work.
Every once in a while the disassembler will interpret things wrong and a chunk of the display will be nonsense.
It's kinda just how it goes, try not to worry if you see it happen.

Also, at the end of our function we can see there's an undefined instruction.
Those will happen sometimes at the end functions.
I'm unclear on why.
It doesn't seem to be for alignment, because going 4 bytes past `0x0800_00E8` to `0x0800_00EC` would make things *less* aligned.
Still, I guess it's not really a big deal when it happens.
We've got so much ROM space available that an occasional 2 or 4 bytes extra won't really break the bank.
