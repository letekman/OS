
use crate::io_ports;
use crate::PIC8259;
use crate::idt;


pub extern "C" fn isr0(stack_frame: &idt::ExceptionStackFrame) {
    unsafe{
        PIC8259::sendEOI(0);
    }

}

pub extern "C" fn isr1(stack_frame: &idt::ExceptionStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);
    unsafe{
        let scancode: u8 = io_ports::inb(0x60);
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
        PIC8259::sendEOI(1);
    }

}

pub extern "C" fn isr2(stack_frame: &idt::ExceptionStackFrame) {
    unsafe{

        println!("isr2 :(");
        PIC8259::sendEOI(2);
        PIC8259::sendEOI(10);
    }    

}


pub extern "C" fn div_zero(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("DIVIDE BY ZERO");
}

pub extern "C" fn debug(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("DEBUG");
}

pub extern "C" fn non_maskable_interrupt(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("NON_MASKABLE_INTERRUPT");
}

pub extern "C" fn breakpoint(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("breakpoint");
}

pub extern "C" fn overflow(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("overflow");
}

pub extern "C" fn bound_range_exceeded(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("bound_range_exceeded");
}

pub extern "C" fn invalid_opcode(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("invalid_opcode");
}

pub extern "C" fn device_not_avaiable(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("device_not_avaiable");
}

pub extern "C" fn double_fault(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("DOUBLE FAULT");
}

pub extern "C" fn invalid_tss(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("INVALID TSS");
}

pub extern "C" fn segment_not_present(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("SEGMENT NOT PRESENT");
}

pub extern "C" fn stack_segment_fault(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("STACK SEGMENT FAULT");
}

pub extern "C" fn general_protect_fault(stack_frame: &idt::ExceptionStackFrame) {
    
    serial_println!("GENERAL PROTECTION FAULT");

}

pub extern "C" fn page_fault(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("PAGE FAULT");
}


pub extern "C" fn x87_floating_point(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("x87_floating_point");
}


pub extern "C" fn alignment_check(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("alignment_check");
}


pub extern "C" fn machine_check(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("machine check");
}

pub extern "C" fn simd(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("simd");
}

pub extern "C" fn virtualization(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("virtualization");
}

pub extern "C" fn security_exception(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("security_exception");
}