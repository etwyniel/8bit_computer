#[cfg(unix)]
pub fn use_color() -> bool {
    use std::os::unix::io::{AsRawFd, RawFd};

    extern "C" {
        fn isatty(fd: RawFd) -> std::os::raw::c_int;
    }
    unsafe { isatty(std::io::stdout().as_raw_fd()) == 1 }
}

#[cfg(not(unix))]
pub fn use_color() -> bool {
    false
}
