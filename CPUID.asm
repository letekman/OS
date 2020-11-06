bits 16

global CheckCPUID
global CheckLongMode

extern PrintString


segment .text
CheckCPUID:    
    ; Check if CPUID is supported by attempting to flip the ID bit (bit 21) in
    ; the FLAGS register. If we can flip it, CPUID is available.
 
    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax
 
    ; Copy to ECX as well for comparing later on
    mov ecx, eax
 
    ; Flip the ID bit
    xor eax, 1 << 21
 
    ; Copy EAX to FLAGS via the stack
    push eax
    popfd
 
    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax
 
    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the ID bit
    ; back if it was ever flipped).
    push ecx
    popfd
 
    ; Compare EAX and ECX. If they are equal then that means the bit wasn't
    ; flipped, and CPUID isn't supported.
    xor eax, ecx
    jz NoCPUID

    ; Check if CPUID supprots long ode check
    mov eax, 0x80000000    ; Set the A-register to 0x80000000.
    cpuid                  ; CPU identification.
    cmp eax, 0x80000001    ; Compare the A-register with 0x80000001.
    jb NoLongModeCheck         ; It is less, there is no long mode.

    ret

CheckLongMode:    
    mov eax, 0x80000001    ; Set the A-register to 0x80000001.
    cpuid                  ; CPU identification.
    test edx, 1 << 29      ; Test if the LM-bit, which is bit 29, is set in the D-register.
    jz NoLongModeSupport 
    ret



NoCPUID:
    mov bx, CPUIDFail
    call PrintString
    hlt


NoLongModeCheck:
    mov bx, NoLongModeCheckString
    call PrintString
    hlt


NoLongModeSupport:
    mov bx, NoLongMode
    call PrintString
    hlt


segment .data

CPUIDFail:
     db 'No CPUID', 0xa, 0xd, 0

NoLongModeCheckString:
     db 'No long mode check supported', 0xa, 0xd, 0

NoLongMode:
     db 'Long Mode is not supported', 0xa, 0xd, 0