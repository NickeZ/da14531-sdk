#![no_std]
#![feature(const_fn_floating_point_arithmetic)]

extern crate alloc;

#[cfg(feature = "expose_bindings")]
pub mod bindings;

#[cfg(not(feature = "expose_bindings"))]
mod bindings;

pub mod allocator;
pub mod app;
pub mod app_modules;
pub mod ble_stack;
pub mod platform;
pub mod stdlib;

pub use paste::paste;

#[cfg(debug_assertions)]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn GPIO_reservations() {}
