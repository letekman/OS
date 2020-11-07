extern gdt_descriptor
extern codeseg
extern dataseg
extern CheckCPUID
extern SetupPaging
extern CheckLongMode

bits 16

section .text

EnterLongMode:
    call CheckCPUID
    call CheckLongMode
    call EnableA20
    cli

    call SetupPaging

    lgdt [gdt_descriptor]

    jmp codeseg:Start64 
 
bits 64

Start64:
    mov ax, dataseg           ; Set the A-register to the data descriptor.
    mov ds, ax                    ; Set the data segment to the A-register.
    mov es, ax                    ; Set the extra segment to the A-register.
    mov fs, ax                    ; Set the F-segment to the A-register.
    mov gs, ax                    ; Set the G-segment to the A-register.
    mov ss, ax                    ; Set the stack segment to the A-register.

    mov edi, 0xB8000              ; Set the destination index to 0xB8000.
    mov rax, 0x1F201F201F201F20   ; Set the A-register to 0x1F201F201F201F20.
    mov ecx, 500  
    rep stosq

    extern _extstart
    jmp _extstart
    
    jmp $


; Simple hack to enable 21th address line so all of the memory
; can be accessed......Supposedly works on most of the machines

bits 16

EnableA20:
    ;input from port 92h to al
    in al, 0x92
    ; set 2nd bit
    or al, 2
    ;output to 92h port
    out 0x92, al
    ret
