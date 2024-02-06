
# Cargo Configuration

We'll want to set two configuration files before we begin.
One you might not have used before, and one that you definitely have.

## `.cargo/config.toml`

[docs](https://doc.rust-lang.org/cargo/reference/config.html)

The first file we'll be using is `.cargo/config.toml`.
You might have never done this before, it's not needed in most Rust projects.
In this file, we basically alter the defaults for when we call `cargo` on the command line.
Since we'll be using `cargo` a whole lot, it's much better to set these things in the file, rahter than trying to remember to set them with every `cargo` usage.

### Default Target

First we want to set the default target for all compilations.
There's two Rust targets that work on the GBA: `thumbv4t-none-eabi` and `armv4t-none-eabi`.

The GBA's CPU is an ARM7TDMI, and that "T" means that "thumb-interworking-is-available".
The CPU will always boot in "ARM state", but after that it can switch to "Thumb state" whenever needed.

ARM code is always 32-bit instructions.
Because the CPU accesses ROM over a 16-bit bus that means that it has to wait for *two* 16-bit accesses to get each ARM instruction out of ROM.
Thumb code, on the other hand, is generally 16-bit instructions.
When executing thumb code stored in ROM, the CPU can get each next instruction in a *single* access.
An individual thumb instruction can't do as much as an individual arm instruction, so sometimes it will take multiple thumb instructions.
Still, on average it's a win for us.

The `thumb` and `arm` parts at the start of the two targets which what the *default* instruction style will be used for all functions.
We want most of our code to be thumb code, so we'll use `thumbv4t-none-eabi`.

```toml
[build]
target = "thumbv4t-none-eabi"
```

### `build-std`

[docs](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) -- **THIS REQUIRES NIGHTLY RUST**

Unfortunately for us, `thumbv4t-none-eabi` is a Tier 3 target.
That means that we'll have to use the unstable `build-std` feature of `cargo` to do our builds.

We also want the `compiler_builtins` crate to use weak-linked intrinsics, so that we can override the intrinsics.

```toml
[unstable]
build-std = ["core"]
build-std-features = ["compiler-builtins-weak-intrinsics"]
```

Having to use Nightly instead of Stable is always slightly unfortunate,
but in practice there's almost never breaking changes to how `build-std` works when updating rust.
I was building GBA projects with `build-std` for about two years without any problems.
At one point there was a build error, and I just needed to flip on the weak intrinsics flag given above.
Since then, I've had no other problems.
So, don't worry too much about getting breaks from using Nightly.

### Target Runner And Rustflags

[docs](https://doc.rust-lang.org/cargo/reference/config.html#targettriplerunner)

We also need to set the "runner" for `thumbv4t-none-eabi`.
This is the program that will run your executables.
It's specifically intended for cross-compilation situations like ours, when you need to run the program in an emulator.
Here, you just put the name of your GBA emulator, and make sure it's in the system path.
On Linux and Mac, mGBA provides both "mgba" and "mgba-qt" binaries.
The "-qt" version has GUI controls.
On Windows, "mgba.exe" is provided, but on my machine I just renamed "mgba.exe" to "mgba-qt.exe", to match the Linux and Mac names.

Also, we need to set `rustflags` to have an extra argument for the linker: the linking script to use.
Linkers are actually quite configurable, and we will need something *other than* the default linking to make GBA stuff.
Instead, we'll provide our own script.

The `-T` argument to the linker lets you give a path to the linker script.
For this project, I'm going to keep any linker scripts we ever make in a `linker_scripts/` folder.
Most of our programs will be for the "standard" boot sequence, so I'll just call it `standard_boot.ld`.
Linker scripts are just text files, but they end in `.ld` by convention.
The binary name of a linker is usually `ld` (which stands for "Link eDitor"), or something similar like `lld` or `gold` or `mold` or whatever else.

```toml
[target.thumbv4t-none-eabi]
runner = "mgba-qt"
rustflags = ["-Clink-arg=-Tlinker_scripts/standard_boot.ld"]
```

**ALSO:**
If you want to use the `objdump` that comes with GNU Binutils, you should set the GNU Binutils linker to be used.
If you mix LLVM's linker and GNU's `objdump` it won't understand where the symbols are, so it won't be able to show what functions are where.
Instead, `objdump` will just show each output section as a single, continuous stream of assembly.
It'll be fairly unreadable, and at least you'll know that something is wrong right away.

Setting the linker to use is part of the `target.thumbv4t-none-eabi` config.

```toml
[target.thumbv4t-none-eabi]
linker = "arm-none-eabi-ld"
```

## `Cargo.toml`

The `Cargo.toml` file is used in every Rust project, so you've definitely seen this one.

In addition to all the normal settings (package name, package version, etc), you're stronly advised to make the `dev` profile use full optimizations.

Rust without optimizations generates hilariously bad code.
Like it's almost a joke.
The only thing keeping normal dev builds of Rust programs able to run at all is the fact that a desktop or server CPU runs at several gigahertz.
The GBA has a very weak CPU, just 16Mhz, so we *always* need full optimization.

Personally, I don't like having to type `--release` every single time I use `cargo` to get an optimized build, so I just turn on full optimization for the `dev` profile.

```toml
[profile.dev]
opt-level = 3
```

Don't worry, using `opt-level=3` won't make the rebuild times on your GBA programs jump super high or anything.
It'll be totally fine.

## Optional: `-Ztrap-unreachable=no`

Normally, LLVM will put a "trap" instruction at unreachable points of functions.
This is, I guess, to try and make sure the program crashes if the program control flow goes out of bounds or something.
I don't know why exactly, but what I do know is that it won't crash the GBA like intended.
The GBA's undefined instruction handler just returns without doing anything.
So, the trap instructions don't help us in any way at all.

If you want, you can put `-Ztrap-unreachable=no` as one of the flags in the `rustflags` list.
This gets rid of those trap instructions.
It's nice, but also not actually strictly necessary.

If, some day, `build-std` was Stable, but `trap-unreachable=no` still wasn't, we could just forget this option and use Stable.
