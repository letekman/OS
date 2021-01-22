use alloc::alloc::{GlobalAlloc, Layout};
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
        let mut size: u32 = layout.size() as u32;
        let segment_header_size: u32 = size_of::<MemorySegment>() as u32;
        let mut cur_segment: *mut MemorySegment = ALLOCATOR.first_segment;
        
        loop {
            if (*cur_segment).size >= size && (*cur_segment).is_free == true {
                if (*cur_segment).size > size + segment_header_size {
                    let mut new_segment: *mut MemorySegment = (add_bytes!(cur_segment, size + segment_header_size)) as *mut MemorySegment;
                    (*new_segment).size = (*cur_segment).size - segment_header_size - size;
                    (*new_segment).next_segment = (*cur_segment).next_segment as *mut MemorySegment;
                    (*new_segment).is_free = true;
                    
                    (*cur_segment).next_segment = new_segment;
                    (*cur_segment).size = size;

                }
                (*cur_segment).is_free = false;
                


                let x = add_bytes!(cur_segment, segment_header_size) as *mut u8;


                return x;
            }
            if(*cur_segment).next_segment == 0 as *mut MemorySegment {

                panic!("END OF HEAP");
            }
            cur_segment = (*cur_segment).next_segment;
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        
        let segment_header_size = size_of::<MemorySegment>();
        let segment: *mut MemorySegment = sub_bytes!(_ptr, segment_header_size) as *mut MemorySegment;
        (*segment).is_free = true;
        merge_free_segments();
    }

}

#[global_allocator]
static mut ALLOCATOR: Aloc = Aloc::new(0x100000, 0x100000);