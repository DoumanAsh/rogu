#[cfg(feature = "ufmt")]
compile_error!("Unable to use ufmt writers with log feature");

pub struct Logger;

impl Into<crate::Level> for log::Level {
    #[inline(always)]
    fn into(self) -> crate::Level {
        match self {
            log::Level::Error => crate::Level::ERROR,
            log::Level::Warn => crate::Level::WARN,
            log::Level::Info => crate::Level::INFO,
            log::Level::Debug => crate::Level::DEBUG,
            log::Level::Trace => crate::Level::TRACE,
        }
    }
}

impl Into<log::LevelFilter> for crate::Level {
    #[inline(always)]
    fn into(self) -> log::LevelFilter {
        match self {
            crate::Level::NONE => log::LevelFilter::Off,
            crate::Level::ERROR => log::LevelFilter::Error,
            crate::Level::WARN => log::LevelFilter::Warn,
            crate::Level::INFO => log::LevelFilter::Info,
            crate::Level::DEBUG => log::LevelFilter::Debug,
            crate::Level::TRACE => log::LevelFilter::Trace,
        }
    }
}

impl log::Log for Logger {
    #[inline(always)]
    fn enabled(&self, meta: &log::Metadata) -> bool {
        crate::is_enabled(meta.level().into())
    }

    #[inline]
    fn log(&self, record: &log::Record) {
        let level = record.level().into();

        if crate::is_enabled(level) {
            let mut out = match level {
                crate::Level::ERROR => crate::Out::error(""),
                crate::Level::WARN => crate::Out::warn(""),
                crate::Level::INFO => crate::Out::info(""),
                crate::Level::DEBUG => crate::Out::debug(""),
                crate::Level::TRACE => crate::Out::trace(""),
                _ => panic!("Unexpected log level"),
            };

            #[cfg(not(feature = "ufmt"))]
            {
                use core::fmt::Write;
                let _ = match (record.file(), record.line()) {
                    (Some(file), Some(line)) => core::writeln!(out, "- [{}:{}] - {}", file, line, record.args()),
                    _ => core::writeln!(out, "{}", record.args()),
                };
            }
        }
    }

    #[inline(always)]
    fn flush(&self) {
    }
}

pub fn init(level: log::LevelFilter) {
    static LOGGER: Logger = Logger;
    log::set_max_level(level);
    let _ = log::set_logger(&LOGGER);
}
