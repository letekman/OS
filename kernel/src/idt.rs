
use crate::io_ports;
use crate::isr_handlers;
use crate::PIC8259;
pub const IDT_ENTRIES: usize = 256;

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
    fn init_entry(handler: u64, ist: u8) -> IDTEntry {
        let offset_start = (handler  & 0xffff) as u16;
        let selector: u16 = 0x8;
        let ist: u8 = ist;
        let type_attr: u8 = 0x8e;
        let offset_middle =  ((handler  & 0xffff0000) >> 16) as u16;
        let offset_end =  (handler  >> 32) as u32;
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
pub unsafe fn load_idt() -> () {

    let idtr =  IDTR {
        limit: 256 * 16 - 1,
        location: _IDT.as_ptr() as u64
    };
    println!("&IDTR: {:#X}", &idtr as *const _ as u64);
    let mut ptr: *mut u16 = &idtr as *const _ as *mut u16;
    println!("IDTR1: {:#X}", *(ptr) as u64);
    println!("IDTR2: {:#X}", *(ptr.offset(1)) as u64);
    println!("IDTR.location: {:#X}", idtr.location as u64);
    println!("isr1: {:#X}", isr_handlers::isr1 as u64);
    
    PIC8259::init(0x20, 0x28);
    init_idt();

    llvm_asm!(
        "
        lidt [$0]
        sti
        " :: "r"(ptr) : "memory" : "intel", "volatile");
    println!("end of load_idt");
}

pub unsafe fn init_idt() -> () {
    println!("in init_idt");
    for i in 0..IDT_ENTRIES{

        _IDT[i] = IDTEntry::EMPTY;
    }
    _IDT[0] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[1] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[2] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[3] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[4] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[5] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[6] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[7] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[8] = IDTEntry::init_entry(isr_handlers::double_fault as u64, 0);
    _IDT[9] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[10] = IDTEntry::init_entry(isr_handlers::invalid_tss as u64, 0);
    _IDT[11] = IDTEntry::init_entry(isr_handlers::segment_not_present as u64, 0);
    _IDT[12] = IDTEntry::init_entry(isr_handlers::stack_segment_fault as u64, 0);
    _IDT[13] = IDTEntry::init_entry(isr_handlers::general_protect_fault as u64, 0);
    _IDT[14] = IDTEntry::init_entry(isr_handlers::page_fault as u64, 0);
    _IDT[15] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[16] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[17] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[18] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[19] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[20] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[21] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[22] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[23] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[24] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[25] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[26] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[27] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[28] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[29] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);
    _IDT[30] = IDTEntry::init_entry(isr_handlers::div_zero as u64, 0);

    //PIC8259::clear_irq_mask(0);
    PIC8259::clear_irq_mask(1);
    //PIC8259::clear_irq_mask(2);
    //_IDT[32] = IDTEntry::init_entry(isr_handlers::isr0 as u64, 0);
    _IDT[33] = IDTEntry::init_entry(isr_handlers::isr1 as u64, 0);
    //_IDT[34] = IDTEntry::init_entry(isr_handlers::isr2 as u64, 0);
}