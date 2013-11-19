use std::libc::c_int;
use std::str::raw;

pub static LC_ALL:      c_int = 0;
pub static LC_COLLATE:  c_int = 1;
pub static LC_CTYPE:    c_int = 2;
pub static LC_MONETARY: c_int = 3;
pub static LC_NUMERIC:  c_int = 4;
pub static LC_TIME:     c_int = 5;
pub static LC_MESSAGES: c_int = 6;

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
    use std::libc::{c_char,c_int};
    extern "C" {
        pub fn setlocale(category:c_int, locale:*c_char) -> *c_char;
    }
}

pub fn setlocale(lc: category, locale: &str) -> ~str {
    unsafe {
        let ret = locale.with_c_str(|buf| { self::native::setlocale(lc as c_int, buf) });
        raw::from_c_str(ret)
    }
}
