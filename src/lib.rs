//!Logging library, plain and simple.
//!
//!Simple logger.
//!
//!Minimal customization, builtin loggers:
//!
//!- Web - Uses console API;
//!- Android Log;
//!- C stdio - writes to stdout/stderr;
//!
//!## Features
//!
//!Note: all features must be used in binary code, not in library
//!
//!#### Loggers
//!
//!- `ufmt` - Enables [ufmt](https://github.com/japaric/ufmt) traits instead of core's `fmt`
//!- `std` - Enables `std` usage, adding ability to use timestamps (not used in Web and Android)
//!- `log` - Enables `log` usage, adding `log` logs to the output.
//!
//!#### Compile time macros
//!
//!- `level_error_off` - Disables `error!` in debug mode;
//!- `level_warn_off` - Disables `warn!` in debug mode;
//!- `level_info_off` - Disables `info!` in debug mode;
//!- `level_debug_off` - Disables `debug!` in debug mode;
//!- `level_trace_off` - Disables `trace!` in debug mode;
//!- `level_all_off`- Disables all macros in debug mode;
//!
//!- `release_level_error_off` - Disables `error!` in release mode;
//!- `release_level_warn_off` - Disables `warn!` in release mode;
//!- `release_level_info_off` - Disables `info!` in release mode;
//!- `release_level_debug_off` - Disables `debug!` in release mode;
//!- `release_level_trace_off` - Disables `trace!` in release mode;
//!- `release_level_all_off`- Disables all macros in release mode;

#![warn(missing_docs)]
#![no_std]

#[cfg(feature = "log")]
mod rust_log;
#[cfg(feature = "std")]
mod time;
mod data;
mod out;
mod rt;
#[doc(hidden)]
pub use out::Out;
#[cfg(feature = "ufmt")]
mod ufmt;
#[cfg(not(feature = "ufmt"))]
mod cor;

#[cfg(feature = "ufmt")]
use crate::ufmt::derive::uDebug as Debug;

use core::sync::atomic::{AtomicU8, Ordering};

static LEVEL: AtomicU8 = AtomicU8::new(0);

///Logging levels
#[repr(u8)]
#[derive(Copy, Eq, Debug)]
pub enum Level {
    #[doc(hidden)]
    NONE = 0,
    /// Designates very serious errors.
    ERROR = 1,
    /// Designates hazardous situations.
    WARN,
    /// Designates useful information.
    INFO,
    /// Designates lower priority information.
    DEBUG,
    /// Designates very low priority, often extremely verbose, information.
    TRACE,
}

impl Clone for Level {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl PartialEq for Level {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}

///Sets logging level
pub fn set_level(level: Level) {
    rt::init();
    LEVEL.store(level as u8, Ordering::Relaxed);

    #[cfg(feature = "log")]
    {
        rust_log::init(level.into());
    }
}

#[inline]
///Checks if logging is enabled
pub fn is_enabled(level: Level) -> bool {
    LEVEL.load(Ordering::Relaxed) >= level as u8
}
