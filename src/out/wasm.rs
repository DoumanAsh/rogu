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

pub struct Console {
    fun: fn(&str),
    buffer: [u8; 4096],
    len: usize,
}

impl Console {
    fn new(fun: fn(&str), level: &'static str, location: &'static str) -> Self {
        let mut res = Self {
            fun,
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
        Self::new(error, data::level::ERROR, location)
    }

    pub fn warn(location: &'static str) -> Self {
        Self::new(warn, data::level::WARN, location)
    }

    pub fn info(location: &'static str) -> Self {
        Self::new(info, data::level::INFO, location)
    }

    pub fn debug(location: &'static str) -> Self {
        Self::new(debug, data::level::DEBUG, location)
    }

    pub fn trace(location: &'static str) -> Self {
        Self::new(trace, data::level::TRACE, location)
    }

    pub fn flush(&mut self) {
        let text = unsafe {
            core::str::from_utf8_unchecked(&self.buffer[..self.len])
        };
        (self.fun)(text);
        self.len = 0;
    }
}

impl ufmt::uWrite for Console {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        //Yeah, how about to not write so much actually?
        debug_assert!(text.len() <= self.buffer.len());

        if self.len == self.buffer.len() || self.len + text.len() > self.buffer.len() {
            self.flush();
        }

        let write_len = cmp::min(self.buffer.len(), text.len());
        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), self.buffer.as_mut_ptr().add(self.len), write_len);
        }
        self.len += write_len;

        if self.buffer[self.len - 1] == b'\n' {
            self.len -= 1;
            self.flush();
        }

        Ok(())
    }
}
