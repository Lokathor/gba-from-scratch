/* THIS LINKER SCRIPT FILE IS RELEASED TO THE PUBLIC DOMAIN (SPDX: CC0-1.0) */

ENTRY(_start)

MEMORY {
  ewram (w!x) : ORIGIN = 0x2000000, LENGTH = 256K
  iwram (w!x) : ORIGIN = 0x3000000, LENGTH = 32K
  rom (rx)    : ORIGIN = 0x8000000, LENGTH = 32M
}

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
