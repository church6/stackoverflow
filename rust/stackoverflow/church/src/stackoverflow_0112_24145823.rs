// @filename   : stackoverflow_0112_24145823.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/24145823
// @title      : How do I convert a C string into a Rust string and back via FFI?

// --------------------------------
/*
extern crate libc;

use libc::c_char;
use std::ffi::c_char;
use std::ffi::CStr;
use std::str;
extern "C" {
    fn hello() -> *const c_char;
}
*/
// --------------------------------

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            /*
            let c_buf: *const c_char = unsafe { hello() };
            let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
            let str_slice: &str = c_str.to_str().unwrap();
            let str_buf: String = str_slice.to_owned();
            */
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer3 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
pub fn test() {
    _enter!();
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
