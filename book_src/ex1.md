
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
