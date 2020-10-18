use wasm_bindgen::prelude::wasm_bindgen;

use core::{mem, ptr, cmp};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn trace(s: &str);
}

use crate::data;

const BUFFER_SIZE: usize = 4096;

pub struct Console {
    fun: fn(&str),
    buffer: mem::MaybeUninit<[u8; BUFFER_SIZE]>,
    len: usize,
}

impl Console {
    fn new(fun: fn(&str), level: &'static str, location: &'static str) -> Self {
        let mut res = Self {
            fun,
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
        Self::new(error, data::level::ERROR, location)
    }

    #[inline(always)]
    pub fn warn(location: &'static str) -> Self {
        Self::new(warn, data::level::WARN, location)
    }

    #[inline(always)]
    pub fn info(location: &'static str) -> Self {
        Self::new(info, data::level::INFO, location)
    }

    #[inline(always)]
    pub fn debug(location: &'static str) -> Self {
        Self::new(debug, data::level::DEBUG, location)
    }

    #[inline(always)]
    pub fn trace(location: &'static str) -> Self {
        Self::new(trace, data::level::TRACE, location)
    }

    pub fn flush(&mut self) {
        let text = unsafe {
            let buffer = core::slice::from_raw_parts(self.as_ptr(), self.len);
            core::str::from_utf8_unchecked(buffer)
        };
        (self.fun)(text);
        self.len = 0;
    }

    pub fn write_text(&mut self, text: &str) {
        //Yeah, how about to not write so much actually?
        debug_assert!(text.len() <= BUFFER_SIZE);

        if self.len == BUFFER_SIZE || self.len + text.len() > BUFFER_SIZE {
            self.flush();
        }

        let write_len = cmp::min(BUFFER_SIZE, text.len());
        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), self.as_mut_ptr().add(self.len), write_len);
        }
        self.len += write_len;

        if unsafe { *self.as_ptr().add(self.len - 1) } == b'\n' {
            self.len -= 1;
            self.flush();
        }
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uWrite for Console {
    type Error = core::convert::Infallible;

    #[inline]
    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        self.write_text(text);

        Ok(())
    }
}

#[cfg(not(feature = "ufmt"))]
impl core::fmt::Write for Console {
    #[inline]
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        self.write_text(text);

        Ok(())
    }
}
