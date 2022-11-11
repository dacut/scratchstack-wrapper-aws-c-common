#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, dead_code)]
#![warn(clippy::all)]
//! Rust wrapper for the `aws-c-common` library. For testing purposes only.
//! For interacting with AWS services, use the `aws-sdk-rust` crate instead.

use libc::{time_t, tm, FILE};

#[cfg(unix)]
use libc::{pthread_cond_t, pthread_mutex_t, pthread_rwlock_t, pthread_t};

#[cfg(any(target_os = "ios", target_os = "macos"))]
use core_foundation::base::CFAllocatorRef;

#[cfg(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "64"))]
const __PTHREAD_ONCE_SIZE__: usize = 8;

#[cfg(all(any(target_os = "ios", target_os = "macos"), target_pointer_width = "32"))]
const __PTHREAD_ONCE_SIZE__: usize = 4;

#[cfg(any(target_os = "ios", target_os = "macos"))]
#[repr(C)]
pub struct pthread_once_t {
    __sig: std::ffi::c_long,
    __opaque: [std::ffi::c_char; __PTHREAD_ONCE_SIZE__],
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub type pthread_once_t = std::ffi::c_int;

#[cfg(windows)]
use std::ffi::c_void;

pub type va_list = *mut std::ffi::c_void;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod sync_ptr;
pub use sync_ptr::SyncPtr;

#[cfg(test)]
mod tests;