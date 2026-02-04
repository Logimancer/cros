ENTRY(_start)

SECTIONS
{
    /* The RPi4B (AArch64) firmware loads the kernel at this address */
    . = 0x80000;

    /* Executable code section */
    .text :
    {
        /* Ensure the entry point (_start) is at the very beginning */
        KEEP(*(.text._start))
        *(.text .text.*)
    }

    /* Read-only data (constants) */
    .rodata : ALIGN(8)
    {
        *(.rodata .rodata.*)
    }

    /* Initialized data */
    .data : ALIGN(8)
    {
        *(.data .data.*)
    }

    /* Uninitialized data (zero-initialized) */
    .bss (NOLOAD) : ALIGN(16)
    {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(16);
        __bss_end = .;
    }

    /* Reserve 128KB for the stack */
    .stack (NOLOAD) : ALIGN(16)
    {
        . += 0x20000; 
        __stack_top = .;
    }

    /* Discard sections that are not needed for bare metal */
    /DISCARD/ :
    {
        *(.comment)
        *(.gnu*)
        *(.note*)
        *(.eh_frame*)
    }
}
