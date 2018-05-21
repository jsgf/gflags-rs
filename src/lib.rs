//! gflags binding for Rust
//!
//! Gflags is based on setting up global variables which are auto-populated from argc/argv.
//! This relies on several things Rust does not have:
//! - global variables (well, not idiomatically)
//! - global constructors
//!
//! gflags docs: https://gflags.github.io/gflags/

#[macro_use]
extern crate lazy_static;

use std::ptr;
use std::sync::Mutex;

mod ffi;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Type {
    Bool,   // B
    Int32,  // I
    Int64,  // I64
    UInt64, // U64
    Double, // D
    String, // S
}

impl Type {
    fn short(&self) -> &'static str {
        use Type::*;

        match self {
            Bool => "B",
            Int32 => "I",
            Int64 => "I64",
            UInt64 => "U64",
            Double => "D",
            String => "S",
        }
    }

    fn long(&self) -> &'static str {
        use Type::*;

        match self {
            Bool => "bool",
            Int32 => "int32",
            Int64 => "int64",
            UInt64 => "uint64",
            Double => "double",
            String => "std::string",
        }
    }
}

lazy_static! {
    static ref FLAGS: Mutex<Vec<ffi::FlagRegisterer>> = Mutex::new(Vec::new());
}

union RawValue {
    b: bool,
    int: i32,
    uint: u32,
    uint64: u64,
    s: *mut ffi::std_string,
}

pub struct Flag<T> {
    ty: Type,
    raw: RawValue,
    defl: RawValue,
}

pub fn add_flag(name: &'static str, ty: Type, help: &'static str, filename: &'static str) {
    let mut flags = FLAGS.lock().unwrap();

    let flag = ffi::FlagRegisterer::new(
        name,
        ty.long(),
        help,
        filename,
        ptr::null_mut(), // XXX FIXME
        ptr::null_mut(), // XXX FIXME
    );

    flags.push(flag)
}

/// Usage:
/// 
/// type(name, help [, default]),
/// ```
/// define_flags! {
///     bool(woogle, "do some woogling"),
///     u32(plerp, "plerp that thing", "ungulate"),
/// }
/// ```
/// Flags are defined into a sub-module `flags`, so you would access
/// `woogle` as `flags::woogle` to get the value.
//#[macro_export]
//macro_rules! define_flags {
//    (bool ( $name:tt, $help:expr ) ) => {
//        
//    };
//}