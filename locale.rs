// warning about unused static definitions seems over-zealous to me.
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use libc::c_int;
use std::str::raw;

pub static LC_ALL:      c_int = 0;
pub static LC_COLLATE:  c_int = 1;
pub static LC_CTYPE:    c_int = 2;
pub static LC_MONETARY: c_int = 3;
pub static LC_NUMERIC:  c_int = 4;
pub static LC_TIME:     c_int = 5;
pub static LC_MESSAGES: c_int = 6;

#[repr(i32)]
pub enum category {
    all = LC_ALL,
    collate = LC_COLLATE,
    ctype = LC_CTYPE,
    monetary = LC_MONETARY,
    numeric = LC_NUMERIC,
    time = LC_TIME,
    messages = LC_MESSAGES,
}

mod native {
    use libc::{c_char,c_int};
    extern "C" {
        pub fn setlocale(category:c_int, locale:*const c_char) -> *const c_char;
    }
}

pub fn setlocale(lc: category, locale: &str) -> String {
    unsafe {
        let ret = locale.with_c_str(|buf| { self::native::setlocale(lc as c_int, buf) });
        ::std::string::raw::from_buf(ret as *const u8)
    }
}
