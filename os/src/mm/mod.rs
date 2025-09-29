//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{kernel_stack_position, MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{copy_from_user, copy_to_user, PTEFlags, PageTable};
pub use page_table::{translated_byte_buffer, PageTableEntry};

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

/// 判断地址是否按align对齐
pub fn is_aligned(value: usize, align: usize) -> bool {
    assert!(align.is_power_of_two(), "对齐大小必须是 2 的幂次方");
    value & (value - 1) == 0
}

/// 得到向上对齐的值
pub fn align_up(value: usize, align: usize) -> usize {
    assert!(align.is_power_of_two(), "对齐大小必须是 2 的幂次方");
    (value + align - 1) & !(align - 1)
}

/// 得到向下对齐的值
pub fn align_down(value: usize, align: usize) -> usize {
    assert!(align.is_power_of_two(), "对齐大小必须是 2 的幂次方");
    value & !(align - 1)
}
