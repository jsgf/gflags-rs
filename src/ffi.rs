#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};

#[repr(C)]
pub struct str_slice<'a> {
    base: *const c_char,
    len: usize,
    _phantom: PhantomData<&'a str>,
}

impl<'a> From<&'a str> for str_slice<'a> {
    fn from(s: &'a str) -> Self {
        str_slice {
            base: s.as_ptr() as *const c_char,
            len: s.len(),
            _phantom: PhantomData,
        }
    }
}

pub struct FlagRegisterer(*mut GFLAGS_NAMESPACE_FlagRegisterer);

unsafe impl Send for FlagRegisterer {}
unsafe impl Sync for FlagRegisterer {}

impl FlagRegisterer {
    pub fn new(
        name: &'static str,
        ty: &'static str,
        help: &'static str,
        filename: &'static str,
        current: *mut c_void,
        defvalue: *mut c_void,
    ) -> Self {
        let ptr = unsafe {
            flag_registerer(
                name.into(),
                ty.into(),
                help.into(),
                filename.into(),
                current,
                defvalue,
            )
        };
        FlagRegisterer(ptr)
    }
}

impl Drop for FlagRegisterer {
    fn drop(&mut self) {
        unsafe {
            free_flag_registerer(self.0);
        }
    }
}
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
