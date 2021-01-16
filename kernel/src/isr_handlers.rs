
use crate::io_ports;
use crate::PIC8259;
use crate::idt;
use crate::cli;


pub extern "C" fn isr0(stack_frame: &idt::ExceptionStackFrame) {
    unsafe{
        //print!(".");
        PIC8259::sendEOI(0);
    }

}

pub extern "C" fn isr1(stack_frame: &idt::ExceptionStackFrame) {
    unsafe{
        let scancode: u8 = io_ports::inb(0x60);
        use crate::keyboard;
        if let Some(key) = keyboard::getKey(scancode){
            cli::print_to_cli(key);
            // if let Some(c) = key.toChar(){
            //     //print!("{}", c);
            // }
        }
        PIC8259::sendEOI(1);
    }

}

pub extern "C" fn isr2(stack_frame: &idt::ExceptionStackFrame) {
    unsafe{
        PIC8259::sendEOI(2);
        PIC8259::sendEOI(10);
    }    

}


pub extern "C" fn div_zero(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("{:#?}", stack_frame);
    loop{}
}

pub extern "C" fn debug(stack_frame: &idt::ExceptionStackFrame){ 
    println!("DEBUG");
}

pub extern "C" fn non_maskable_interrupt(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("NON_MASKABLE_INTERRUPT: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn breakpoint(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("breakpoint: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn overflow(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("overflow: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn bound_range_exceeded(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("bound_range_exceeded: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn invalid_opcode(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("invalid_opcode: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn device_not_avaiable(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("device_not_avaiable: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn double_fault(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("DOUBLE FAULT: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn invalid_tss(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("INVALID TSS: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn segment_not_present(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("SEGMENT NOT PRESENT: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn stack_segment_fault(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("STACK SEGMENT FAULT: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn general_protect_fault(stack_frame: &idt::ExceptionStackFrame) {
    
    serial_println!("GENERAL PROTECTION FAULT: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn page_fault(stack_frame: &idt::ExceptionStackFrame, error_code: u64){ 
    serial_println!("PAGE FAULT: {:#?}", stack_frame);
    loop{}
}


pub extern "C" fn x87_floating_point(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("x87_floating_point: {:#?}", stack_frame);
    loop{}
}


pub extern "C" fn alignment_check(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("alignment_check: {:#?}", stack_frame);
    loop{}
}


pub extern "C" fn machine_check(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("machine check: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn simd(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("simd: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn virtualization(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("virtualization: {:#?}", stack_frame);
    loop{}
}

pub extern "C" fn security_exception(stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("security_exception: {:#?}", stack_frame);
    loop{}
}