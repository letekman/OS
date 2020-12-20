use crate::isr_handlers;
use crate::PIC8259;




pub const IDT_ENTRIES: usize = 256;
pub type HandlerFunc = extern "C" fn() -> !;
macro_rules! save_scratch_registers {
    () => {
        llvm_asm!("push rax
              push rcx
              push rdx
              push rsi
              push rdi
              push r8
              push r9
              push r10
              push r11
        " :::: "intel", "volatile");
    }
}

macro_rules! restore_scratch_registers {
    () => {
        llvm_asm!("pop r11
              pop r10
              pop r9
              pop r8
              pop rdi
              pop rsi
              pop rdx
              pop rcx
              pop rax
            " :::: "intel", "volatile");
    }
}
macro_rules! handler {
    ($name: expr) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                save_scratch_registers!();
                llvm_asm!("mov rdi, rsp
                      add rdi, 9*8 // calculate exception stack frame pointer
                      call ${0:c}"
                      :: "i"($name as
                             extern "C" fn(&ExceptionStackFrame))
                      : "rdi" : "intel", "volatile");

                restore_scratch_registers!();
                llvm_asm!("
                      iretq"
                      :::: "intel", "volatile");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}
macro_rules! handler_with_error_code {
    ($name: expr) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                save_scratch_registers!();
                llvm_asm!("
                    mov rsi, [rsp + 9*8] // load error code into rsi
                    mov rdi, rsp
                    add rdi, 10*8 // calculate exception stack frame pointer
                    sub rsp, 8 // align the stack pointer
                    call ${0:c}
                    " :: "i"($name as 
                             extern "C" fn(&ExceptionStackFrame, u64))
                    : "rdi","rsi" : "intel");
                restore_scratch_registers!();
                llvm_asm!("
                    add rsp, 8 // pop error code
                    iretq" 
                    :::: "intel", "volatile");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}
#[derive(Debug)]
#[repr(C)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}
#[repr(C, packed)]
struct IDTR {
    limit: u16,
    location: u64
}
extern {
    static mut _IDT: [IDTEntry; IDT_ENTRIES];
} 
#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
 struct IDTEntry {
    offset_start: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_middle: u16,
    offset_end: u32,
    zero: u32
}
impl IDTEntry{
    pub const EMPTY: IDTEntry = IDTEntry {
        offset_start: 0,
        selector: 0,
        ist: 0,
        type_attr: 0,
        offset_middle: 0,
        offset_end: 0,
        zero: 0
    };
    fn init_entry(handler: HandlerFunc, ist: u8) -> IDTEntry {
        let pointer = handler as u64;
        let offset_start = (pointer  & 0xffff) as u16;
        let selector: u16 = 0x8;
        let ist: u8 = ist;
        let type_attr: u8 = 0x8e;
        let offset_middle =  ((pointer  & 0xffff0000) >> 16) as u16;
        let offset_end =  (pointer  >> 32) as u32;
        let zero :u32 = 0;
    
        IDTEntry {
            offset_start,
            selector,
            ist: ist,
            type_attr: type_attr,
            offset_middle: offset_middle,
            offset_end: offset_end,
            zero: zero
        }
    }
} 
pub unsafe fn print_rbp() -> () {
    let mut base_pointer: *const usize;
    llvm_asm!("mov rax, rbp" : "={rax}"(base_pointer) ::: "intel");
    serial_println!("RBP: {}", base_pointer as usize);
}
pub unsafe fn print_rsp() -> () {
    let mut base_pointer: *const usize;
    llvm_asm!("mov rax, rsp" : "={rax}"(base_pointer) ::: "intel");
    serial_println!("RSP: {}", base_pointer as usize);
}
pub unsafe fn load_idt() -> () {

    let idtr =  IDTR {
        limit: 256 * 16 - 1,
        location: _IDT.as_ptr() as u64
    };
    let mut ptr: *mut u16 = &idtr as *const _ as *mut u16;
    // println!("IDTR1: {:#X}", *(ptr) as u64);
    // println!("IDTR2: {:#X}", *(ptr.offset(1)) as u64);
    // println!("IDTR.location: {:#X}", idtr.location as u64);
    // println!("isr1: {:#X}", isr1 as u64);
    // print!("YO{}", 2);
    
    PIC8259::init(0x20, 0x28);
    init_idt();
    llvm_asm!(
        "
        lidt [$0]
        sti
        " :: "r"(ptr) : "memory" : "intel", "volatile");
}

pub unsafe fn init_idt() -> () {
    for i in 0..IDT_ENTRIES{

        _IDT[i] = IDTEntry::EMPTY;
    }
    _IDT[0] = IDTEntry::init_entry(handler!(isr_handlers::div_zero), 0);
    _IDT[1] = IDTEntry::init_entry(handler!(isr_handlers::debug), 0);
    _IDT[2] = IDTEntry::init_entry(handler!(isr_handlers::non_maskable_interrupt), 0);
    _IDT[3] = IDTEntry::init_entry(handler!(isr_handlers::breakpoint), 0);
    _IDT[4] = IDTEntry::init_entry(handler!(isr_handlers::overflow), 0);
    _IDT[5] = IDTEntry::init_entry(handler!(isr_handlers::bound_range_exceeded), 0);
    _IDT[6] = IDTEntry::init_entry(handler!(isr_handlers::invalid_opcode), 0);
    _IDT[7] = IDTEntry::init_entry(handler!(isr_handlers::device_not_avaiable), 0);
    _IDT[8] = IDTEntry::init_entry(handler!(isr_handlers::double_fault), 0);
    _IDT[9] = IDTEntry::init_entry(handler!(isr_handlers::debug), 0);
    _IDT[10] = IDTEntry::init_entry(handler!(isr_handlers::invalid_tss), 0);
    _IDT[11] = IDTEntry::init_entry(handler!(isr_handlers::segment_not_present), 0);
    _IDT[12] = IDTEntry::init_entry(handler!(isr_handlers::stack_segment_fault), 0);
    _IDT[13] = IDTEntry::init_entry(handler!(isr_handlers::general_protect_fault), 0);
    _IDT[14] = IDTEntry::init_entry(handler_with_error_code!(isr_handlers::page_fault), 0);
    _IDT[15] = IDTEntry::init_entry(handler!(isr_handlers::debug), 0);
    _IDT[16] = IDTEntry::init_entry(handler!(isr_handlers::x87_floating_point), 0);
    _IDT[17] = IDTEntry::init_entry(handler!(isr_handlers::alignment_check), 0);
    _IDT[18] = IDTEntry::init_entry(handler!(isr_handlers::machine_check), 0);
    _IDT[19] = IDTEntry::init_entry(handler!(isr_handlers::simd), 0);
    _IDT[20] = IDTEntry::init_entry(handler!(isr_handlers::virtualization), 0);
    _IDT[30] = IDTEntry::init_entry(handler!(isr_handlers::security_exception), 0);

    PIC8259::clear_irq_mask(0);
    PIC8259::clear_irq_mask(1);
    PIC8259::clear_irq_mask(2);
    _IDT[32] = IDTEntry::init_entry(handler!(isr_handlers::isr0), 0);
    _IDT[33] = IDTEntry::init_entry(handler!(isr_handlers::isr1), 0);
    _IDT[34] = IDTEntry::init_entry(handler!(isr_handlers::isr2), 0);
}
