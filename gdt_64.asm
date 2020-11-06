bits 16

global gdt_descriptor
global codeseg
global dataseg
global SetupGDT64

gdt_start:

gdt_nulldesc:
    dq 0
    
gdt_codedesc:
    ; Go to https://wiki.osdev.org/GDT for explanation

    ; This Describes the span that GDT can access
    dw 0xFFFF ; Setting the GDT_Limit to Max
    dw 0x0000 ; Setting the GDT_Base to Min
    db 0x00 ; GDT_Base cnd.

    ; Setting The Access Byte
    ; Bits: |Pr| Privl |S|Ex|DC|RW|Ac|
    ;         1    2    1  1  1  1  1  bit 
    db 10011010b
    
    ; Setting The Flags
    ; Bits |GR|Sz|0|0| Then next bits are GDT_LIMIT cnd.
    db 10101111b

    
    db 0x00; GDT_BASE cnd.
gdt_datadesc:
    ; The Same as gdt_codedesc with exception of
    ; Executable Bit being 0
    dw 0xFFFF ; Setting the GDT_Limit to Max
    dw 0x0000 ; Setting the GDT_Base to Min
    db 0x00 ; GDT_Base cnd.
    
    ; Setting The Access Byte
    db 10010010b ; Here's the change
    ;      -
    
    db 10101111b; Setting The Flags and GDT_LIMIT cnd.

    db 0x00; GDT_BASE cnd.
gdt_end:

gdt_descriptor:
    ; This defines gdt description structure
    gdt_size:
        ; The size is the size of the table subtracted by 1. This is 
        ; because the maximum value of size is 65535, 
        ; while the GDT can be up to 65536 bytes
        dw gdt_end - gdt_start - 1

    gdt_start_address:
        dd gdt_start

codeseg equ gdt_codedesc - gdt_start
dataseg equ gdt_datadesc - gdt_start


SetupGDT64:
    mov [gdt_codedesc + 6], byte 10101111b
    mov [gdt_datadesc + 6], byte 10101111b
    ret