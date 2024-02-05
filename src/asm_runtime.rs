macro_rules! blank_header_space {
  () => {
    concat! {
      "b 1f\n",
      ".space 0xE0\n",
      "1:\n",
    }
  };
}

macro_rules! copy_words_r0r1r2r3 {
  (dest=$dest:literal, src=$src:literal, count=$count:literal $(,)?) => {
    concat!(
      concat!("ldr r0, =", $dest, "\n"),
      concat!("ldr r1, =", $src, "\n"),
      concat!("ldr r2, =", $count, "\n"),
      "1:\n",
      "subs    r2, r2, #4\n",
      "ldrge   r3, [r1], #4\n",
      "strge   r3, [r0], #4\n",
      "bgt     1b\n",
    )
  };
}

macro_rules! zero_words_r0r1r2 {
  (start=$start:literal, count=$count:literal $(,)?) => {
    concat!(
      concat!("ldr r0, =", $start, "\n"),
      "mov r1, #0\n",
      concat!("ldr r0, =", $count, "\n"),
      "1:\n",
      "subs    r2, r2, #4\n",
      "strge   r1, [r0], #4\n",
      "bgt     1b\n",
    )
  };
}

/// Rom header and assembly initialization.
///
/// ## Safety
/// You are **never** allowed to call this function from Rust.
#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".text._start"]
unsafe extern "C" fn _start() -> ! {
  core::arch::asm! {
    blank_header_space!(),

    copy_words_r0r1r2r3!(
      dest="_iwram_start",
      src="_iwram_position_in_rom",
      count="_iwram_word_copy_count",
    ),

    copy_words_r0r1r2r3!(
      dest="_ewram_start",
      src="_ewram_position_in_rom",
      count="_ewram_word_copy_count",
    ),

    zero_words_r0r1r2!(
      start = "_bss_start",
      count = "_bss_word_clear_count",
    ),

    // Set Assembly Interrupt Handler
    "ldr r1, ={asm_irq_handler}",
    "mov r12, #0x04000000",
    "str r1, [r12, #-4]",

    // Call `main`
    "ldr r0, =main",
    "bx r0",
    options(noreturn),
    asm_irq_handler = sym asm_irq_handler,
  }
}

/// The assembly runtime interrupt handler.
///
/// ## Safety
/// You are **never** allowed to call this function from Rust.
#[naked]
#[instruction_set(arm::a32)]
#[link_section = ".iwram.asm_irq_handler"]
unsafe extern "C" fn asm_irq_handler() {
  // On Entry: r0 = 0x0400_0000 (mmio_base)
  core::arch::asm! {
    // Read/Update IE and IF

    // Read/Update BIOS_IF

    // return to the BIOS handler
    "bx lr",
    options(noreturn)
  }
}
