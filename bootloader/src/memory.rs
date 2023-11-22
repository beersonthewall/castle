use spin::mutex::Mutex;

/// Physical Address
pub struct PhysAddr(u64);

/// Virtual Address
pub struct VirtAddr(u64);

/// Allocates virtual pages to frames of physical memory.
struct PageFrameAllocator {
    inner: Mutex<PFAInner>
}

/// Page frame allocator inner struct.
struct PFAInner {}

struct PageFrameBitmap {
    
}

struct BootstrapPFA {}

impl PageFrameAllocator {
    fn new() -> Self {
	Self {
	    inner: Mutex::new(PFAInner::new())
	}
    }
}

impl PFAInner {
    fn new() -> Self {
	Self {}
    }
}

pub fn init() {
    
}
