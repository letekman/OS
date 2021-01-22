
use crate::io_ports;
use crate::pic8259;
use crate::idt;
use crate::cli;


pub extern "C" fn isr0(_stack_frame: &idt::ExceptionStackFrame) {
    unsafe{
        pic8259::send_eoi(0);
    }

}

pub extern "C" fn isr1(_stack_frame: &idt::ExceptionStackFrame) {
    unsafe{
        let scancode: u8 = io_ports::inb(0x60);
        use crate::keyboard;
        if let Some(key) = keyboard::get_key(scancode){
            cli::print_to_cli(key);
        }
        pic8259::send_eoi(1);
    }

}

pub extern "C" fn isr2(_stack_frame: &idt::ExceptionStackFrame) {
    unsafe{
        pic8259::send_eoi(2);
        pic8259::send_eoi(10);
    }    

}


pub extern "C" fn div_zero(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("{:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn debug(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("DEBUG");
}

pub extern "C" fn non_maskable_interrupt(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("NON_MASKABLE_INTERRUPT: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn breakpoint(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("breakpoint: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn overflow(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("overflow: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn bound_range_exceeded(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("bound_range_exceeded: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn invalid_opcode(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("invalid_opcode: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn device_not_avaiable(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("device_not_avaiable: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn double_fault(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("DOUBLE FAULT: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn invalid_tss(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("INVALID TSS: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn segment_not_present(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("SEGMENT NOT PRESENT: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn stack_segment_fault(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("STACK SEGMENT FAULT: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn general_protect_fault(_stack_frame: &idt::ExceptionStackFrame) {
    
    serial_println!("GENERAL PROTECTION FAULT: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn page_fault(_stack_frame: &idt::ExceptionStackFrame, _error_code: u64){ 
    serial_println!("PAGE FAULT: {:#?}", _stack_frame);
    loop{}
}


pub extern "C" fn x87_floating_point(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("x87_floating_point: {:#?}", _stack_frame);
    loop{}
}


pub extern "C" fn alignment_check(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("alignment_check: {:#?}", _stack_frame);
    loop{}
}


pub extern "C" fn machine_check(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("machine check: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn simd(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("simd: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn virtualization(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("virtualization: {:#?}", _stack_frame);
    loop{}
}

pub extern "C" fn security_exception(_stack_frame: &idt::ExceptionStackFrame){ 
    serial_println!("security_exception: {:#?}", _stack_frame);
    loop{}
}