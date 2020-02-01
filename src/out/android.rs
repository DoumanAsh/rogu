use core::{cmp, mem, ptr};
use crate::data;

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(i32)]
enum LogPriority {
    UNKNOWN = 0,
    DEFAULT = 1,
    VERBOSE = 2,
    DEBUG = 3,
    INFO = 4,
    WARN = 5,
    ERROR = 6,
    FATAL = 7,
    SILENT = 8,
}

const MSG_MAX_LEN: usize = 4000;
const TAG: &[u8; 5] = b"Rust\0";

#[link(name = "log")]
extern "C" {
    pub fn __android_log_write(prio: i32, tag: *const i8, text: *const i8) -> i32;
}

pub struct Log {
    prio: LogPriority,
    buffer: [u8; MSG_MAX_LEN + 1],
    len: usize,
}

impl Log {
    fn new(prio: LogPriority, level: &'static str, location: &'static str) -> Self {
        let mut res = Self {
            prio,
            buffer: unsafe { mem::MaybeUninit::uninit().assume_init() },
            len: 0,
        };

        unsafe {
            ptr::copy_nonoverlapping(level.as_ptr(), res.buffer.as_mut_ptr(), level.len())
        }
        res.len += level.len();

        unsafe {
            ptr::copy_nonoverlapping(location.as_ptr(), res.buffer.as_mut_ptr().add(res.len), location.len())
        }
        res.len += location.len();

        res
    }

    pub fn error(location: &'static str) -> Self {
        Self::new(LogPriority::ERROR, data::level::ERROR, location)
    }

    pub fn warn(location: &'static str) -> Self {
        Self::new(LogPriority::WARN, data::level::WARN, location)
    }

    pub fn info(location: &'static str) -> Self {
        Self::new(LogPriority::INFO, data::level::INFO, location)
    }

    pub fn debug(location: &'static str) -> Self {
        Self::new(LogPriority::DEBUG, data::level::DEBUG, location)
    }

    pub fn trace(location: &'static str) -> Self {
        Self::new(LogPriority::VERBOSE, data::level::TRACE, location)
    }

    pub fn flush(&mut self) {
        self.buffer[self.len - 1] = 0;

        unsafe {
            __android_log_write(self.prio as i32, TAG.as_ptr() as *const _, self.buffer.as_ptr() as *const _);
        }

        self.len = 0;
    }

    pub fn write_text(&mut self, text: &str) {
        //Yeah, how about to not write so much actually?
        debug_assert!(text.len() <= MSG_MAX_LEN);

        if MSG_MAX_LEN == self.len {
            self.flush();
        }
        let write_len = cmp::min(MSG_MAX_LEN - self.len, text.len());

        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), self.buffer.as_mut_ptr().add(self.len), write_len);
        }
        self.len += write_len;

        if self.buffer[self.len - 1] == b'\n' {
            self.flush();
        }
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uWrite for Log {
    type Error = core::convert::Infallible;

    #[inline]
    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        self.write_text(text);

        Ok(())
    }
}

#[cfg(not(feature = "ufmt"))]
impl core::fmt::Write for Log {
    #[inline]
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        self.write_text(text);

        Ok(())
    }
}
