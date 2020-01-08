///C stdlib baesd writer into stdout/stderr

use crate::data;

pub struct FdWriter {
    fd: u8,
    level: &'static str,
    location: &'static str,
    header: bool,
}

impl FdWriter {
    const fn new(fd: u8, level: &'static str, location: &'static str) -> Self {
        Self {
            fd,
            level,
            location,
            header: false,
        }
    }

    pub const fn error(location: &'static str) -> Self {
        Self::new(2, data::level::ERROR, location)
    }

    pub const fn warn(location: &'static str) -> Self {
        Self::new(2, data::level::WARN, location)
    }

    pub const fn info(location: &'static str) -> Self {
        Self::new(1, data::level::INFO, location)
    }

    pub const fn debug(location: &'static str) -> Self {
        Self::new(1, data::level::DEBUG, location)
    }

    pub const fn trace(location: &'static str) -> Self {
        Self::new(1, data::level::TRACE, location)
    }
}

impl ufmt::uWrite for FdWriter {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        let fd = self.fd.into();

        unsafe {
            if !self.header {
                libc::write(fd, self.level.as_ptr() as *const _, self.level.len() as _);

                #[cfg(feature = "std")]
                {
                    let time = crate::time::get();
                    let time_str = time.as_str();

                    libc::write(fd, time_str.as_ptr() as *const _, time_str.len() as _);
                }

                libc::write(fd, self.location.as_ptr() as *const _, self.location.len() as _);
                self.header = true;
            }
            libc::write(fd, text.as_ptr() as *const _, text.len() as _);
        }

        Ok(())
    }
}
