bits 16
global SECOND_STAGE_ORIGIN
extern PrintString
extern ReadDisk
extern BOOT_DISK



global _start
section .text
    _start:
    ;Stack init
    mov [BOOT_DISK], dl

    mov bp, 0x7c00
    mov sp, bp

    mov bx, BootInitiated
    call PrintString

    call ReadDisk



segment .data
    BootInitiated:
        db 'Boot started', 0xa, 0xd, 0

SECOND_STAGE_ORIGIN equ 0x8000


