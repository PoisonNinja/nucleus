OUTPUT_FORMAT("elf64-x86-64")
VMA = 0xFFFFFFFF80000000;
ENTRY(bootstrap32)
SECTIONS
{
	. = 0x100000;
    __kernel_start = .;

	.bootstrap :
	{
		*(.bootstrap)
		*(.trampoline)
	}

	. += VMA;

    .text ALIGN (0x1000) : AT(ADDR(.text) - VMA)
    {
    	*(.text)
	}

    .rodata ALIGN (0x1000) : AT(ADDR(.rodata) - VMA)
    {
        *(.rodata)
    }

    .data ALIGN (0x1000) : AT(ADDR(.data) - VMA)
    {
        *(.data)
    }

	.eh_frame ALIGN(0x1000) : AT(ADDR(.eh_frame) - VMA)
   	{
   	    *(.eh_frame)
   	}

    .bss ALIGN(0x1000) : AT(ADDR(.bss) - VMA)
    {
        *(.bss)
        *(COMMON)
    }

    /DISCARD/ :
    {
       	*(.comment)
    }

    __kernel_end = . - VMA;
}