use core::alloc::{GlobalAlloc, Layout};

use crate::bindings::{ke_check_malloc, ke_free, ke_malloc, KE_MEM_NON_RETENTION};

pub struct Da14531Allocator;

impl Da14531Allocator {}

unsafe impl GlobalAlloc for Da14531Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        #[cfg(debug_assertions)]
        if !ke_check_malloc(layout.size() as u32, KE_MEM_NON_RETENTION as u8) {
            panic!("Cannot allocate");
        }
        ke_malloc(layout.size() as u32, KE_MEM_NON_RETENTION as u8) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { ke_free(ptr as *mut cty::c_void) }
    }
}
