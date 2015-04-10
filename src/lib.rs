extern crate libc;

use libc::{c_char,c_int,size_t};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::str;

#[link(name="c")]
extern {
    fn posix_openpt(mode: c_int) -> c_int;
    fn grantpt(fd: c_int) -> c_int;
    fn unlockpt(fd: c_int) -> c_int;
    fn ptsname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    fn ptsname(fd: c_int) -> *const c_char;
}

pub const RDWR: i32 = 2;
pub const NOCTTY: i32 = 256;

pub fn open(mode: i32) -> RawFd {
    unsafe {
        posix_openpt(mode)
    }
}

pub fn grant(fd: RawFd) -> bool {
    unsafe {
        grantpt(fd) == 0
    }
}

pub fn unlock(fd: RawFd) -> bool {
    unsafe {
        unlockpt(fd) == 0
    }
}

pub fn name_r<'a>(fd: RawFd) -> &'a str {
    // TODO clean this up
    unsafe {
        // create a buffer that will be passed as
        // a string pointer to the FFI function
        let mut pts_name: Vec<u8> = vec!();
        for _ in 0..64 {
            pts_name.push(0u8);
        }
        let buf = CString::from_vec_unchecked(pts_name).as_ptr();

        // most implementations I've seen for ptsname(int) use 64 characters
        // for the buffer, so I will assume this is long enough for now
        ptsname_r(fd, buf as *mut i8, 64u64);
        let name = CStr::from_ptr(buf).to_bytes();
        str::from_utf8(name).ok().unwrap()
    }
}

// unsafe due to concurrency issues with the ptsname(int)
// call storing the string in static memory
pub unsafe fn name<'a>(fd: RawFd) -> &'a str {
    let pts_name: *const i8 = ptsname(fd);
    let name = CStr::from_ptr(pts_name).to_bytes();
    str::from_utf8(name).ok().unwrap()
}

