use linked_list_allocator::LockedHeap;

// Your existing ALLOCATOR handles the "metadata" (The HashMap nodes, the String names)
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

// Your new CODE_ALLOCATOR handles the "actual bytes" of the functions

pub struct CodeAllocator {
    start: usize,
    size: usize,
    offset: core::sync::atomic::AtomicUsize,
}

pub static CODE_POOL: CodeAllocator = CodeAllocator {
    start: 0x200_000, // Starting at 2MB, well above your 1MB heap
    size: 0x100_000,  // 1MB of space for dynamic commands
    offset: core::sync::atomic::AtomicUsize::new(0),
};

impl CodeAllocator {
    pub fn new() -> Self {
        Self {
            start: 0x200_000,
            size: 0x100_000,
            offset: core::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn allocate(&self, len: usize) -> Option<*mut u8> {
        // Align to 4 bytes for ARM instructions
        let aligned_len = (len + 3) & !3;
        let current_offset = self.offset.fetch_add(aligned_len, core::sync::atomic::Ordering::SeqCst);
        
        if current_offset + aligned_len <= self.size {
            Some((self.start + current_offset) as *mut u8)
        } else {
            None
        }
    }
}
