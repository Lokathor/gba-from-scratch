
# The `_start` fn

```rust
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".text._start"]
unsafe extern "C" fn _start() -> ! {
  core::arch::asm! {
    "nop",
    options(noreturn)
  }
}
```

```rust
core::arch::global_asm! {
  ".section .text._start",
  ".code 32",
  ".global _start",
  "_start:",
  "nop",
  ".code 16",
}
```
