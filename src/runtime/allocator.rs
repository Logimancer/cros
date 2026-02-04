use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    // On Pi4, 0x80_000 is kernel start. Let's start the heap at 1MB
    // and give it 1MB of space for now.
    let heap_start = 0x100_000 as *mut u8; 
    let heap_size = 0x100_000; // 1MB
    
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}
