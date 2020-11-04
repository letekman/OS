OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start)
OUTPUT(
	bootloader.elf
)
INPUT (
	bootloader.o
	print.o
    readDisk.o
)

SECTIONS
{
    . = 0x7C00;
    .text : SUBALIGN(2) {
        bootloader.o(.text);
        *(.text);
    }
   
    /* Place the data after the code */
    .data : SUBALIGN(2) {
        *(.data);
        *(.rodata*);
    }

    /* Place the boot signature at LMA/VMA 0x7DFE */
    .sig 0x7DFE : {
        SHORT(0xaa55);
        . +=  0x3E00;
    } 
}
