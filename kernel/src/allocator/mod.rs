mod heap;

/// Size of kernel heap.
pub const KERNEL_HEAP_SIZE: usize = 1024 * 1024;

/// Global heap allocator.
#[global_allocator]
static HEAP_ALLOCATOR: heap::Allocator = heap::Allocator;