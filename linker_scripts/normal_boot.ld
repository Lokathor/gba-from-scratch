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
