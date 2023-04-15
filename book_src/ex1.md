
# Basics

Let's program some stuff to run on the GBA.

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
D:\dev\gba-from-scratch>cargo build --example ex1 --target thumbv4t-none-eabi
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
D:\dev\gba-from-scratch>rustup target add thumbv4t-none-eabi
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
// main.rs
#![no_std]

fn main() {
  //
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
```

Now we get a new, different error when we try to build:

```
> cargo build --example ex1 --target thumbv4t-none-eabi -Z build-std=core
   Compiling gba_from_scratch v0.1.0 (/Users/dg/gba-from-scratch)
error: requires `start` lang_item

error: could not compile `gba_from_scratch` (example "ex1") due to previous error
```
