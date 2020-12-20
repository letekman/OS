use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::ptr::null;
use core::mem::size_of;


macro_rules! add_bytes {
    ($ptr:ident, $offset:expr) => {
        $ptr as u32 + $offset as u32
    };
}
macro_rules! sub_bytes {
    ($ptr:ident, $offset:expr) => {
        $ptr as u32 - $offset as u32
    };
}



extern {
    static _HEAP_START: u32;
    static _HEAP_SIZE: u32;
}

pub unsafe fn init_heap() {
    (*ALLOCATOR.first_segment).size = ALLOCATOR.heap_size - size_of::<*mut MemorySegment>() as u32;
    (*ALLOCATOR.first_segment).next_segment = 0 as *mut MemorySegment;
    (*ALLOCATOR.first_segment).is_free = true;
}

pub unsafe fn print_heap(){
    let mut cur_segment: *mut MemorySegment = ALLOCATOR.first_segment;
    serial_println!("PRINTING HEAP\n");
    while cur_segment != 0 as *mut MemorySegment {
        serial_println!("new node:");
        serial_println!("address: {:p} ", cur_segment);
        serial_println!("{:#x?}", *cur_segment);
        cur_segment = (*cur_segment).next_segment;
    }
}

pub unsafe fn merge_free_segments(){

    let mut cur_segment: *mut MemorySegment = ALLOCATOR.first_segment;
    while (*cur_segment).next_segment != 0 as *mut MemorySegment {
        if (*cur_segment).is_free && (*(*cur_segment).next_segment).is_free {
            (*cur_segment).size +=  (*(*cur_segment).next_segment).size + size_of::<MemorySegment>() as u32;
            (*cur_segment).next_segment = (*(*cur_segment).next_segment).next_segment;
        }
        if  (*cur_segment).next_segment == 0 as *mut MemorySegment {
            break;
        }
        cur_segment = (*cur_segment).next_segment;
    }
}

#[derive(Debug)]
struct MemorySegment {
    size: u32,
    next_segment: *mut MemorySegment,
    is_free: bool
}
pub struct Aloc{
    first_segment: *mut MemorySegment,
    heap_size: u32
}

unsafe impl Send for Aloc {}
unsafe impl Sync for Aloc {}
impl Aloc{
    pub const fn new(heap_start: u32, heap_size: u32) -> Aloc {
        Aloc{first_segment: heap_start as *mut MemorySegment, heap_size: heap_size}
    }
    
}

unsafe impl GlobalAlloc for Aloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let remainder = layout.size() % layout.align();
        let mut size: u32 = (layout.size() - remainder) as u32;
        if remainder != 0 {
            size += layout.align() as u32;
        }
        let pointer_size: u32 = size_of::<*mut MemorySegment>() as u32;
        let segment_header_size: u32 = size_of::<MemorySegment>() as u32;
        let mut cur_segment: *mut MemorySegment = ALLOCATOR.first_segment;
        
        loop {
            // serial_println!("cur_ptr: {:p}, size {:x}",cur_segment, (*cur_segment).size );
            // serial_println!("cur_segment: {:x?}, layout: {:x?}",*cur_segment, layout);
            // serial_println!();
            if (*cur_segment).size >= size && (*cur_segment).is_free == true {
                
                // serial_println!("size1: {:#x}, size2: {:#x}", (*cur_segment).size, size);
                // serial_println!("cur_segmentptr: {:p}",cur_segment);
                (*cur_segment).is_free = false;
                //serial_println!("cur_segmentAAAA: {:x?}",*cur_segment);

                if (*cur_segment).size - segment_header_size > size {
                    let mut new_segment: *mut MemorySegment = (add_bytes!(cur_segment, size + segment_header_size)) as *mut MemorySegment;
                    //serial_println!("cur_segment: {:x?}\n cur_ptr: {:p}\n next_ptr: {:p}\n new_segment: {:x?}\n layout {:#x?}",*cur_segment, cur_segment,new_segment, *new_segment,  layout);
                    (*new_segment).size = (*cur_segment).size - segment_header_size - size;
                    (*new_segment).next_segment = 0 as *mut MemorySegment;
                    (*new_segment).is_free = true;

                    (*cur_segment).next_segment = new_segment;
                    (*cur_segment).size = size;
                    
                serial_println!("cur: {:x?}",(*cur_segment));

                }


                let x = add_bytes!(cur_segment, segment_header_size) as *mut u8;

                serial_println!("x: {:p}",x);

                return x;
            }
            if(*cur_segment).next_segment == 0 as *mut MemorySegment {

                panic!("END OF HEAP");
            }
            // serial_println!("loopin\n\n");
            cur_segment = (*cur_segment).next_segment;
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let segment_header_size = size_of::<MemorySegment>();
        let segment: *mut MemorySegment = sub_bytes!(_ptr, segment_header_size) as *mut MemorySegment;
        serial_println!("cur_segmentDEALLOC: {:p}",segment);
        serial_println!("cur_segmentDEALLOC: {:x?}",*segment);
        (*segment).is_free = true;

        merge_free_segments();
        
    }

}

use lazy_static::lazy_static;

#[global_allocator]
static mut ALLOCATOR: Aloc = Aloc::new(0x100000, 0x100000);