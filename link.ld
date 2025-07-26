ENTRY(_start)

MEMORY {
  /* RAM: 16384 bytes (16KB) from 0x0000 to 0x3FFF */
  /* Console : 0x5000 */ 
  RAM : ORIGIN = 0x0000, LENGTH = 16K
}

SECTIONS {
  /* Set start address */
  . = ORIGIN(RAM);

  /* Code section */
  .text : {
    *(.text._start)      /* entry point */
    *(.text*)            /* all code */
  } > RAM

  /* Read-only data */
  .rodata : ALIGN(4) {
    *(.srodata .srodata.*)
    *(.rodata*)
  } > RAM

  /* Initialized data */
  .data : ALIGN(4) {
    _sidata = LOADADDR(.data);
    _sdata = .;
    /* Must be called __global_pointer$ for linker relaxations to work. */
    PROVIDE(__global_pointer$ = . + 0x800);
    *(.sdata .sdata.* .sdata2 .sdata2.*);
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > RAM AT > RAM

  /* Zero-initialized data (bss) */
  .bss (NOLOAD) : {
    _sbss = .;
    *(.sbss .sbss.* .bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  PROVIDE(_stack_end = ORIGIN(RAM) + LENGTH(RAM));
  PROVIDE(_stack_start = _stack_end - 2048);  /* 2 KiB stack */
  /DISCARD/ : {
    *(.eh_frame*)  /* Rust unwinding info (not needed here) */
  }
}