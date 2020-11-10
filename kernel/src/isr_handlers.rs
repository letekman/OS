
use crate::io_ports;
use crate::PIC8259;

macro_rules! pushall {
    () => {{
        llvm_asm!("
        push rax
        push rbx
        push rcx
        push rdx
        push rsi
        push rdi
        push rbp
        push r8
        push r9
        push r10
        push r11
        push r12
        push r13
        push r14
        push r15

        " :::: "intel", "volatile");
    }};
}

macro_rules! popall {
    () => {{
        llvm_asm!("
        pop r15
        pop r14
        pop r13
        pop r12
        pop r11
        pop r10
        pop r9
        pop r8
        pop rbp
        pop rdi
        pop rsi
        pop rdx
        pop rcx
        pop rbx
        pop rax
        " :::: "intel", "volatile");
    }};
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn isr0() -> () {
    pushall!();
    PIC8259::sendEOI(0);
    popall!();
    llvm_asm!("iretq");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn isr1() -> () {
    pushall!();
    println!("{}",io_ports::inb(0x60));
    serial_println!("ISR1 DZIALA");

    PIC8259::sendEOI(1);
    popall!();

    llvm_asm!("iretq");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn isr2() -> () {
    pushall!();
    println!("isr2 :(");
    PIC8259::sendEOI(2);
    PIC8259::sendEOI(10);
    popall!();
    llvm_asm!("iretq");
}

#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn div_zero() -> () {
    serial_println!("DIVIDE BY ZERO");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn debug() -> () {
    serial_println!("DEBUG");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn non_maskable_interrupt() -> () {
    serial_println!("NON_MASKABLE_INTERRUPT");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn breakpoint() -> () {
    serial_println!("breakpoint");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn overflow() -> () {
    serial_println!("overflow");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn bound_range_exceeded() -> () {
    serial_println!("bound_range_exceeded");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn invalid_opcode() -> () {
    serial_println!("invalid_opcode");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn device_not_avaiable() -> () {
    serial_println!("device_not_avaiable");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn double_fault() -> () {
    serial_println!("DOUBLE FAULT");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn invalid_tss() -> () {
    serial_println!("INVALID TSS");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn segment_not_present() -> () {
    serial_println!("SEGMENT NOT PRESENT");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn stack_segment_fault() -> () {
    serial_println!("STACK SEGMENT FAULT");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn general_protect_fault() -> () {
    serial_println!("GENERAL PROTECTION FAULT");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn page_fault() -> () {
    serial_println!("PAGE FAULT");
}

#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn x87_floating_point() -> () {
    serial_println!("x87_floating_point");
}

#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn alignment_check() -> () {
    serial_println!("alignment_check");
}

#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn machine_check() -> () {
    serial_println!("machine check");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn simd() -> () {
    serial_println!("simd");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn virtualization() -> () {
    serial_println!("virtualization");
}
#[no_mangle]
#[inline(always)]
#[naked]
pub unsafe extern "C" fn security_exception() -> () {
    serial_println!("security_exception");
}