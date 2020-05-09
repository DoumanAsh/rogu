///C stdlib baesd writer into stdout/stderr

use crate::data;
use core::{mem, ptr};

pub struct FdWriter {
    fd: u8,
    buffer: mem::MaybeUninit<[u8; 4096]>,
    len: usize,
}

impl FdWriter {
    fn new(fd: u8, level: &'static str, location: &'static str) -> Self {
        let mut res = Self {
            fd,
            buffer: mem::MaybeUninit::uninit(),
            len: 0,
        };

        unsafe {
            ptr::copy_nonoverlapping(level.as_ptr(), res.buffer_as_mut_ptr(), level.len())
        }
        res.len += level.len();

        #[cfg(feature = "std")]
        {
            let time = crate::time::get();
            let time_str = time.as_str();

            unsafe {
                ptr::copy_nonoverlapping(time_str.as_ptr(), res.buffer_as_mut_ptr().add(res.len), time_str.len())
            }
            res.len += time_str.len();
        }

        unsafe {
            ptr::copy_nonoverlapping(location.as_ptr(), res.buffer_as_mut_ptr().add(res.len), location.len())
        }
        res.len += location.len();

        res
    }

    fn buffer(&self) -> &[u8; 4096] {
        unsafe {
            &*(self.buffer.as_ptr())
        }
    }

    fn buffer_as_mut_ptr(&mut self) -> *mut u8 {
        unsafe {
            (*(self.buffer.as_mut_ptr())).as_mut_ptr()
        }
    }

    #[inline(always)]
    pub fn error(location: &'static str) -> Self {
        Self::new(2, data::level::ERROR, location)
    }

    #[inline(always)]
    pub fn warn(location: &'static str) -> Self {
        Self::new(2, data::level::WARN, location)
    }

    #[inline(always)]
    pub fn info(location: &'static str) -> Self {
        Self::new(1, data::level::INFO, location)
    }

    #[inline(always)]
    pub fn debug(location: &'static str) -> Self {
        Self::new(1, data::level::DEBUG, location)
    }

    #[inline(always)]
    pub fn trace(location: &'static str) -> Self {
        Self::new(1, data::level::TRACE, location)
    }

    fn flush(&mut self) {
        let text = unsafe {
            core::str::from_utf8_unchecked(&self.buffer()[..self.len])
        };
        unsafe {
            libc::write(self.fd.into(), text.as_ptr() as *const _, text.len() as _);
        }
        self.len = 0;
    }

    fn write_text(&mut self, text: &str) {
        //Yeah, how about to not write so much actually?
        debug_assert!(text.len() <= self.buffer().len());

        if self.len == self.buffer().len() || self.len + text.len() > self.buffer().len() {
            self.flush();
        }

        let write_len = core::cmp::min(self.buffer().len(), text.len());
        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), self.buffer_as_mut_ptr().add(self.len), write_len);
        }
        self.len += write_len;

        if self.buffer()[self.len - 1] == b'\n' {
            self.flush();
        }
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uWrite for FdWriter {
    type Error = core::convert::Infallible;

    #[inline]
    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        self.write_text(text);

        Ok(())
    }
}

#[cfg(not(feature = "ufmt"))]
impl core::fmt::Write for FdWriter {
    #[inline]
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        self.write_text(text);

        Ok(())
    }
}
