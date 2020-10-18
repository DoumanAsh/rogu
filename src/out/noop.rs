pub struct Noop {
}

impl Noop {
    #[inline(always)]
    pub fn error(_: &'static str) -> Self {
        Self {
        }
    }

    #[inline(always)]
    pub fn warn(_: &'static str) -> Self {
        Self {
        }
    }

    #[inline(always)]
    pub fn info(_: &'static str) -> Self {
        Self {
        }
    }

    #[inline(always)]
    pub fn debug(_: &'static str) -> Self {
        Self {
        }
    }

    #[inline(always)]
    pub fn trace(_: &'static str) -> Self {
        Self {
        }
    }

}

#[cfg(feature = "ufmt")]
impl ufmt::uWrite for FdWriter {
    type Error = core::convert::Infallible;

    #[inline(always)]
    fn write_str(&mut self, text: &str) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(not(feature = "ufmt"))]
impl core::fmt::Write for FdWriter {
    #[inline(always)]
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        Ok(())
    }
}
