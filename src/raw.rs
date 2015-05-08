use libc::{c_char, c_int, size_t};

#[link(name="c")]
extern {
    pub fn grantpt(fd: c_int) -> c_int;
    pub fn posix_openpt(mode: c_int) -> c_int;
    pub fn ptsname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    pub fn ptsname(fd: c_int) -> *const c_char;
    pub fn unlockpt(fd: c_int) -> c_int;
}
