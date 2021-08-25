/* memory allocation for stm32f3103vct */
/* Linker script for the STM32F303VCT6 */
MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 256K

    /* .bss, .data and the heap go in this region */
    RAM : ORIGIN = 0x20000000, LENGTH = 40K

    /* Core coupled (faster) RAM dedicated to hold the stack */
    CCRAM : ORIGIN = 0x10000000, LENGTH = 8K


/* memory allocation for stm32f1 blue pill

/* Flash memory begins at 0x80000000 and has a size of 64kB*/
/*  FLASH : ORIGIN = 0x08000000, LENGTH = 64K */
/* RAM begins at 0x20000000 and has a size of 20kB*/
/*  RAM : ORIGIN = 0x20000000, LENGTH = 20K */

}

_stack_start = ORIGIN(CCRAM) + LENGTH(CCRAM);

