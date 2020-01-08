use core::{ptr, mem};

const UTF8_OFFSET: u8 = 48;

pub struct TimeDate([u8; 22]);

impl TimeDate {
    pub fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.0)
        }
    }
}

#[inline(always)]
pub fn get() -> TimeDate {
    let now = time::PrimitiveDateTime::now();
    let mut result = TimeDate(unsafe { mem::MaybeUninit::uninit().assume_init() });
    unsafe {
        ptr::write(result.0.as_mut_ptr(), b'[');

        let mut num = now.year();
        for idx in (1..5).rev() {
            ptr::write(result.0.as_mut_ptr().add(idx), num as u8 % 10 + UTF8_OFFSET);
            num /= 10;
        }

        ptr::write(result.0.as_mut_ptr().add(5), b'-');

        let mut num = now.month();
        for idx in (6..8).rev() {
            ptr::write(result.0.as_mut_ptr().add(idx), num % 10 + UTF8_OFFSET);
            num /= 10;
        }

        ptr::write(result.0.as_mut_ptr().add(8), b'-');

        num = now.day();
        for idx in (9..11).rev() {
            ptr::write(result.0.as_mut_ptr().add(idx), num % 10 + UTF8_OFFSET);
            num /= 10;
        }

        ptr::write(result.0.as_mut_ptr().add(11), b' ');

        num = now.hour();
        for idx in (12..14).rev() {
            ptr::write(result.0.as_mut_ptr().add(idx), num % 10 + UTF8_OFFSET);
            num /= 10;
        }

        ptr::write(result.0.as_mut_ptr().add(14), b':');

        num = now.minute();
        for idx in (15..17).rev() {
            ptr::write(result.0.as_mut_ptr().add(idx), num % 10 + UTF8_OFFSET);
            num /= 10;
        }

        ptr::write(result.0.as_mut_ptr().add(17), b':');

        num = now.second();
        for idx in (18..20).rev() {
            ptr::write(result.0.as_mut_ptr().add(idx), num % 10 + UTF8_OFFSET);
            num /= 10;
        }

        ptr::write(result.0.as_mut_ptr().add(20), b']');
        ptr::write(result.0.as_mut_ptr().add(21), b' ');
    }

    result
}
