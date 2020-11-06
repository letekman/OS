bits 16

extern PrintString
extern SECOND_STAGE_ORIGIN

global ReadDisk
global BOOT_DISK_ID

section .text

ReadDisk:    

    mov bx, DiskReadStarted
    call PrintString
    
    mov si, DAPS
    mov ah, 0x42
    mov dl, [BOOT_DISK_ID]
    int 0x13
   
    jc DiskReadFailed

    mov bx, DiskReadSuccess
    call PrintString

    ret

DiskReadFailed:
    mov bx, DiskReadError
    call PrintString
    jmp $


section .data

DAPS:
    db 0x10 
    db 0
    dw 32 ; Number of sectors to read 31 - without boot sector
    dw SECOND_STAGE_ORIGIN ; Address where to store data
    dw 0
    dq 2 ; Absolute number of the start sector - 2 st sector has number 1


DiskReadError:
    db 'Disk read failed', 0xa, 0xd, 0

DiskReadSuccess:
    db 'Disk read successful', 0xa, 0xd, 0

DiskReadStarted:
    db 'Disk read started', 0xa, 0xd, 0

BOOT_DISK_ID db 0


