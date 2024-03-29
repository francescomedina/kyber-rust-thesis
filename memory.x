MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
  CCRAM : ORIGIN = 0x10000000, LENGTH = 8K
}


/* The location of the stack can be overridden using the `_stack_start` symbol.
   By default it will be placed at the end of the RAM region */
/*_stack_start = ORIGIN(CCRAM) + LENGTH(CCRAM);*/
/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ccrambss"]` to the data
   you want to place there. */
/* Note that the section will not be zero-initialized by the runtime! */
SECTIONS {
 .ccrambss (NOLOAD) : ALIGN(4) {
   *(.ccrambss);
   . = ALIGN(4);
 } > CCRAM
};