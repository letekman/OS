[bits 16]
global ReadDisk
global BOOT_DISK
extern PrintString
extern SECOND_STAGE_ORIGIN

section .text

ReadDisk:    

    mov bx, DiskReadStarted
    call PrintString

    mov ah, 0x02 
    mov bx, SECOND_STAGE_ORIGIN
    mov al, 32 ; Read 2048 bytes - 4 sectors (enough for now)
    mov dl, 0; Read from drive in boot_drive
    
    ;Begin reading hard drive from
    mov ch, 0x00; Cylinder 0
    mov dh, 0x00; Head 0

    mov cl, 0x02; Start reading from sector 2

    int 0x13; Interrupt for reading from disk

    jc DiskReadFailed

    mov bx, DiskReadSuccess
    call PrintString

    ret

DiskReadFailed:
    mov bx, DiskReadError
    call PrintString
    jmp $



; Second stage begins at 0x7e00 - 512 bytes (length of first stage)
; After 0x7c00 (origin of 1st stage)


section .data



BOOT_DISK db 0

DiskReadError:
    db 'Disk read failed', 0xa, 0xd, 0

DiskReadSuccess:
    db 'Disk read successful', 0xa, 0xd, 0

DiskReadStarted:
    db 'Disk read started', 0xa, 0xd, 0