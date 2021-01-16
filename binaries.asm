%macro IncBin 2
    SECTION .rodata
    GLOBAL %1
    GLOBAL %1_Size
%1:
    incbin %2
    db 0
    %1_Size: dq %1_Size - %1

%endmacro

IncBin _RootFile, "rootfile.tar"