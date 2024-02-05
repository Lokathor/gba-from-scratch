
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

For each output section desired, you list one or more input section matchers.
The linker then goes through each source file, sorting each input section according to your output section rules.

Here's an example:

```ld
  .rodata : {
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } >rom = 0x00
```

The `.rodata : {` part declares an *output section*.
Within that, the `*(.rodata .rodata.*);` matcher is tested against each file that's input to the linker.
The first `*` outside the parens is where we *could* match against a filename.
It's very rare we ever want to do that, we usually want it to apply to "all files" with `*(   )`.
Within the parens, we use one or more whitespace separated glob matchers.
So `.rodata` means an exact match, and `.rodata.*` means "anything starting with `.rodata.`".

At the end of some of our output section we want in RAM, there's the cryptic line `. = ALIGN(4);`.
This aligns the final address of the section to 4 bytes by inserting any necessary padding.
We'll talk more about alignment later, but bulk data can be copied faster if both the start address and the size are aligned to 4.

Our linker script will have some lines like `_iwram_position_in_rom = .;` which are not in an output section.
These lines assign the current output address (written with just `.`) to the symbol.
These symbols become global values that can be used within our program.
If the program never actually references the symbol they'll just be thrown out, without a problem.
The only concern is if there's a name collision (a name defined more than once), which is why we're using the leading `_`.
By convention, things with a leading underscore are "provided by the toolchain", and the user can't complain much that we used it first.

Since our program is on a game cart being put into the GBA, all of the data will be *physically* "at" a place the ROM when the GBA is turned on.
That's what the `AT>rom` part of some of the output sections means.
Data can also be *logically* allocated somewhere in RAM as well.
This is also known as the "Virtual Memory Address" (VMA) when you look in the ELF info of your compiled program.
That's what the parts like `>iwram` mean.
Lines like `_iwram_start = ABSOLUTE(.);`, that are *inside* of an output section listing, will make the defined symbol have the logical address.

Finally, there's also a bunch of debug metadata things that can occur.
We can just collect any debug info sections from our input files into the same section names in the output file.

```ld
SECTIONS {
  .text : {
    /* Make sure this is the _very_ first ROM entry */
    *(.text._start);
    
    /* Now all other program text can be placed */
    *(.text .text.*);
    . = ALIGN(4);
  } >rom = 0x00

  .rodata : {
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } >rom = 0x00

  . = ALIGN(4);
  _iwram_position_in_rom = .;
  .data : {
    _iwram_start = ABSOLUTE(.);
    
    *(.data .data.*);
    *(.iwram .iwram.*);
    . = ALIGN(4);
    
    _iwram_end = ABSOLUTE(.);
  } >iwram AT>rom = 0x00

  . = ALIGN(4);
  _ewram_position_in_rom = _iwram_position_in_rom + (_iwram_end - _iwram_start);
  .ewram : {
    _ewram_start = ABSOLUTE(.);
    
    *(.ewram .ewram.*);
    . = ALIGN(4);
    
    _ewram_end = ABSOLUTE(.);
  } >ewram AT>rom = 0x00

  . = ALIGN(4);
  _bss_position_in_rom = _ewram_position_in_rom + (_ewram_end - _ewram_start);
  .bss : {
    _bss_start = ABSOLUTE(.);

    *(.bss .bss.*);
    . = ALIGN(4);

    _bss_end = ABSOLUTE(.);
  } >iwram

  _iwram_word_copy_count = (_iwram_end - _iwram_start) / 4;
  _ewram_word_copy_count = (_ewram_end - _ewram_start) / 4;
  _bss_word_clear_count = (_bss_end - _bss_start) / 4;

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
