use crate::alloc::{GlobalAlloc, Layout};

// quark-rt exposes its heap as a GlobalAlloc; std's allocator PAL wants free
// functions, so forward to the static.

#[inline]
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    unsafe { quark_rt::allocator::SYSTEM_ALLOC.alloc(layout) }
}

#[inline]
pub unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
    unsafe { quark_rt::allocator::SYSTEM_ALLOC.alloc_zeroed(layout) }
}

#[inline]
pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    unsafe { quark_rt::allocator::SYSTEM_ALLOC.dealloc(ptr, layout) }
}

#[inline]
pub unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    unsafe { quark_rt::allocator::SYSTEM_ALLOC.realloc(ptr, layout, new_size) }
}
