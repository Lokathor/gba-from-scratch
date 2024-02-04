
# Linker Script

Now that we've told `cargo` that it needs to have the linker use a particular linking script, we better fill in that script.

LLVM's `lld` linker aims to be as compatible as possible with GNU's `ld` linker, so they have the same script file format.

[GNU Binutils docs](https://sourceware.org/binutils/docs/ld/Scripts.html)

We need to specify `ENTRY`, `MEMORY`, and `SECTIONS`.

Since this is a guide more about the GBA than about linker scripts, this part won't go into too much depth.
Read the binutils docs if you need to know more.

## Entry

The `ENTRY` tells the linker what symbol (think: function) should be the entry point of the generated binary.

For actual ROM images it doesn't matter at all, because we'll be stripping the executable metadata anyway.
When we're running the program in an emulator, having a correct starting symbol can help the emulator figure out what to do.

The usual name for the entry point of an embedded thing is `_start` (or something like that).
The `_start` is often written in assembly, and then once it does some minimal setup it calls to `main`, which is written in the high level language.
That fits us just fine, so we'll do that.

```ld
ENTRY(_start)
```

## Memory

The `MEMORY` part gives names and addresses and sizes to all the memory regions that the linker is allowed to allocate things to.

We'll want to be able to assign things to `rom` of course, but we'll also probably want to be able to assign `iwram` and `ewram` data as well.

Having the linker allocate tile and palette data is also possible.
However, usually that's not done within the linker because the program will want to be able to change what's within vram dynamically as the game state changes.
We won't bother to define those regions within our linker script.

```
MEMORY {
  ewram (w!x) : ORIGIN = 0x2000000, LENGTH = 256K
  iwram (w!x) : ORIGIN = 0x3000000, LENGTH = 32K
  rom (rx)    : ORIGIN = 0x8000000, LENGTH = 32M
}
```

## Sections

The `SECTIONS` part lets you define a mapping from "input sections" to "output sections".
For each output section desired, you define one or more input section matchers.
All of the input sections get assigned to an output section, or are discarded, according to these matchers.

We can also assign names to addresses that are computed during all the linking.
Importantly, this lets us know the start and end points of different output regions.

All of the data will be *physically* "at" a place the ROM when the GBA is turned on.
Data can also be *logically* allocated somewhere in RAM as well.
We name the start and end positions of these different regions so that our `_start` function can copy the correct ROM ranges into RAM.

Finally, there's also a bunch of debug metadata things that can occur.
We can just collect any debug info from our input files into the output file, and that's fine.

```ld
SECTIONS {
  .text : {
    /* be sure that the ROM header is the _very_ first */
    *(.text.gba_rom_header);
    
    /* Now all other program text can be placed */
    *(.text .text.*);
    . = ALIGN(4);
  } >rom = 0x00

  .rodata : {
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } >rom = 0x00

  . = ALIGN(4);
  __iwram_position_in_rom = .;
  .data : {
    __iwram_start = ABSOLUTE(.);
    
    *(.data .data.*);
    *(.iwram .iwram.*);
    . = ALIGN(4);
    
    __iwram_end = ABSOLUTE(.);
  } >iwram AT>rom = 0x00

  . = ALIGN(4);
  __ewram_position_in_rom = __iwram_position_in_rom + (__iwram_end - __iwram_start);
  .ewram : {
    __ewram_start = ABSOLUTE(.);
    
    *(.ewram .ewram.*);
    . = ALIGN(4);
    
    __ewram_end = ABSOLUTE(.);
  } >ewram AT>rom = 0x00

  . = ALIGN(4);
  __bss_position_in_rom = __ewram_position_in_rom + (__ewram_end - __ewram_start);
  .bss : {
    __bss_start = ABSOLUTE(.);

    *(.bss .bss.*);
    . = ALIGN(4);

    __bss_end = ABSOLUTE(.);
  } >iwram

  __iwram_word_copy_count = (__iwram_end - __iwram_start) / 4;
  __ewram_word_copy_count = (__ewram_end - __ewram_start) / 4;
  __bss_word_clear_count = (__bss_end - __bss_start) / 4;

  /* rust-lld demands we keep the `section header string table` */
  .shstrtab        0 : { *(.shstrtab) }

  /* debugging sections */
  .stab            0 : { *(.stab) }
  .stabstr         0 : { *(.stabstr) }
  .stab.excl       0 : { *(.stab.excl) }
  .stab.exclstr    0 : { *(.stab.exclstr) }
  .stab.index      0 : { *(.stab.index) }
  .stab.indexstr   0 : { *(.stab.indexstr) }
  .comment         0 : { *(.comment) }
  .debug           0 : { *(.debug) }
  .line            0 : { *(.line) }
  .debug_srcinfo   0 : { *(.debug_srcinfo) }
  .debug_sfnames   0 : { *(.debug_sfnames) }
  .debug_aranges   0 : { *(.debug_aranges) }
  .debug_pubnames  0 : { *(.debug_pubnames) }
  .debug_info      0 : { *(.debug_info) }
  .debug_abbrev    0 : { *(.debug_abbrev) }
  .debug_line      0 : { *(.debug_line) }
  .debug_frame     0 : { *(.debug_frame) }
  .debug_str       0 : { *(.debug_str) }
  .debug_loc       0 : { *(.debug_loc) }
  .debug_macinfo   0 : { *(.debug_macinfo) }
  .debug_weaknames 0 : { *(.debug_weaknames) }
  .debug_funcnames 0 : { *(.debug_funcnames) }
  .debug_typenames 0 : { *(.debug_typenames) }
  .debug_varnames  0 : { *(.debug_varnames) }

  /* discard anything not already mentioned */
  /DISCARD/ : { *(*) }
}
```
