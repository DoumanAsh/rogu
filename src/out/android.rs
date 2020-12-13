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
    buffer: mem::MaybeUninit<[u8; MSG_MAX_LEN + 1]>,
    len: usize,
}

impl Log {
    fn new(prio: LogPriority, level: &'static str, location: &'static str) -> Self {
        let mut res = Self {
            prio,
            buffer: mem::MaybeUninit::uninit(),
            len: 0,
        };

        unsafe {
            ptr::copy_nonoverlapping(level.as_ptr(), res.as_mut_ptr(), level.len())
        }
        res.len += level.len();

        unsafe {
            ptr::copy_nonoverlapping(location.as_ptr(), res.as_mut_ptr().add(res.len), location.len())
        }
        res.len += location.len();

        res
    }

    #[inline(always)]
    fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr() as *const u8
    }

    #[inline(always)]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr() as *mut u8
    }

    #[inline(always)]
    pub fn error(location: &'static str) -> Self {
        Self::new(LogPriority::ERROR, data::level::ERROR, location)
    }

    #[inline(always)]
    pub fn warn(location: &'static str) -> Self {
        Self::new(LogPriority::WARN, data::level::WARN, location)
    }

    #[inline(always)]
    pub fn info(location: &'static str) -> Self {
        Self::new(LogPriority::INFO, data::level::INFO, location)
    }

    #[inline(always)]
    pub fn debug(location: &'static str) -> Self {
        Self::new(LogPriority::DEBUG, data::level::DEBUG, location)
    }

    #[inline(always)]
    pub fn trace(location: &'static str) -> Self {
        Self::new(LogPriority::VERBOSE, data::level::TRACE, location)
    }

    pub fn flush(&mut self) {
        unsafe {
            self.as_mut_ptr().add(self.len - 1).write(0);
            __android_log_write(self.prio as i32, TAG.as_ptr() as *const _, self.as_ptr() as *const _);
        }

        self.len = 0;
    }

    fn copy_text<'a>(&mut self, text: &'a str) -> &'a str {
        let write_len = cmp::min(MSG_MAX_LEN.saturating_sub(self.len), text.len());
        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), self.as_mut_ptr().add(self.len), write_len);
        }
        self.len += write_len;
        &text[write_len..]
    }

    #[cold]
    fn on_text_overflow<'a>(&mut self, mut text: &'a str) -> &'a str {
        loop {
            text = self.copy_text(text);
            self.flush();

            if text.len() <= MSG_MAX_LEN {
                break text
            }
        }
    }

    pub fn write_text(&mut self, mut text: &str) {
        //Yeah, how about to not write so much actually?
        if text.len() > MSG_MAX_LEN {
            self.on_text_overflow(text);
        }

        loop {
            text = self.copy_text(text);

            if text.len() == 0 {
                break;
            } else {
                self.flush();
            }
        }

        if unsafe { *self.as_ptr().add(self.len - 1) } == b'\n' {
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
