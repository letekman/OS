
use crate::io_ports;
const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;
const PIC_EOI: u8 = 0x20;
const INIT_COMMAND: u8 = 0x11;
const MODE_8086: u8 = 0x01;

pub unsafe fn sendEOI(irq: u8) -> () {
    if irq >= 8 {
        io_ports::outb(PIC_EOI, PIC2_COMMAND);
    }
    io_ports::outb(PIC_EOI, PIC1_COMMAND);
}
pub unsafe fn set_irq_mask(mut irq_line: u8) {
    let port: u16;
    let value: u8;
 
    if irq_line < 8 {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        irq_line -= 8;
    }
    value = io_ports::inb(port) | (1 << irq_line);
    io_ports::outb(value, port);      
}
pub unsafe fn clear_irq_mask(mut irq_line: u8) {
    let port: u16;
    let value: u8;

    if irq_line < 8 {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        irq_line -= 8;
    }
    value = io_ports::inb(port) & !(1 << irq_line);
    io_ports::outb(value, port);      
}

pub unsafe fn init(offset1: u8, offset2: u8){
    let mask1: u8 = io_ports::inb(PIC1_DATA);
    let mask2: u8 = io_ports::inb(PIC2_DATA);

    io_ports::outb(INIT_COMMAND, PIC1_COMMAND);
    io_ports::io_wait();
    io_ports::outb(INIT_COMMAND, PIC2_COMMAND);
    io_ports::io_wait();

    io_ports::outb(offset1, PIC1_DATA);
    io_ports::io_wait();
    io_ports::outb(offset2, PIC2_DATA);
    io_ports::io_wait();

    io_ports::outb(4, PIC1_DATA);
    io_ports::io_wait();
    io_ports::outb(2, PIC2_DATA);
    io_ports::io_wait();

    io_ports::outb(MODE_8086, PIC1_DATA);
    io_ports::io_wait();
    io_ports::outb(MODE_8086, PIC2_DATA);
    io_ports::io_wait();
    
    io_ports::outb(mask1, PIC1_DATA);
    io_ports::outb(mask2, PIC2_DATA);
    

}

