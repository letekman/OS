pub const IDT_ENTRIES: u8 = 256;
struct IDTDescr {
    offset_start: uint16,
    selector: uint16,
    ist: uint8,
    type_attr: uint8,
    offset_middle: uint16,
    offset_end: uint32,
    zero: uint32
}


pub fn init_idt() -> () {
    let mut idt: [IDTDescr; IDT_ENTRIES];
    for i in 0..IDT_ENTRIES {
        idt[i].offset_start = 0xffff;
        idt[i].selector = 0x08;
        idt[i].ist = 0;
        idt[i].type_attr = 0x8e;
        idt[i].offset_middle = 0xffff;
        idt[i].offset_middle = 0xffff;
        idt[i].zero = 0;
    }
}
extern "C" fn interrupt_handler() -> () {

}