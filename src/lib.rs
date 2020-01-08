//!Logging library, plain and simple.
//!
//!Simple logger, which uses [ufmt](https://github.com/japaric/ufmt).
//!
//!Minimal customization, builtin loggers:
//!
//!- Web - Uses console API;
//!- Android Log;
//!- C stdio - writes to stdout/stderr;
//!
//!## Features
//!
//!#### Loggers
//!
//!- `std` - Enables `std` usage, adding ability to use timestamps (not used in Web and Android)
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

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
mod time;
mod data;
mod out;
mod rt;
#[doc(hidden)]
pub use out::Out;

use core::sync::atomic::{AtomicU8, Ordering};

static LEVEL: AtomicU8 = AtomicU8::new(0);

///Logging levels
#[repr(u8)]
#[derive(Copy, Eq, ufmt::derive::uDebug)]
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
}

#[inline]
///Checks if logging is enabled
pub fn is_enabled(level: Level) -> bool {
    LEVEL.load(Ordering::Relaxed) >= level as u8
}

#[cfg(any(all(debug_assertions, feature = "level_error_off"), all(not(debug_assertions), feature = "release_level_error_off") ) )]
#[macro_export]
///Writes error log
macro_rules! error {
    ($($arg:tt)*) => {
    }
}

#[cfg(any(all(debug_assertions, not(feature = "level_error_off")), all(not(debug_assertions), not(feature = "release_level_error_off")) ) )]
#[macro_export]
///Writes error log
macro_rules! error {
    ($($arg:tt)*) => {
        if $crate::is_enabled($crate::Level::ERROR) {
            let _ = ufmt::uwriteln!($crate::Out::error(core::concat!("- [", core::file!(), ":", core::line!(), "] - ") ), $($arg)*);
        }
    }
}

#[cfg(any(all(debug_assertions, feature = "level_warn_off"), all(not(debug_assertions), feature = "release_level_warn_off") ) )]
#[macro_export]
///Writes warn log
macro_rules! warn {
    ($($arg:tt)*) => {
    }
}

#[cfg(any(all(debug_assertions, not(feature = "level_warn_off")), all(not(debug_assertions), not(feature = "release_level_warn_off")) ) )]
#[macro_export]
///Writes warn log
macro_rules! warn {
    ($($arg:tt)*) => {
        if $crate::is_enabled($crate::Level::WARN) {
            let _ = ufmt::uwriteln!($crate::Out::warn(core::concat!("- [", core::file!(), ":", core::line!(), "] - ") ), $($arg)*);
        }
    }
}

#[cfg(any(all(debug_assertions, feature = "level_info_off"), all(not(debug_assertions), feature = "release_level_info_off") ) )]
#[macro_export]
///Writes info log
macro_rules! info {
    ($($arg:tt)*) => {
    }
}

#[cfg(any(all(debug_assertions, not(feature = "level_info_off")), all(not(debug_assertions), not(feature = "release_level_info_off")) ) )]
#[macro_export]
///Writes info log
macro_rules! info {
    ($($arg:tt)*) => {
        if $crate::is_enabled($crate::Level::INFO) {
            let _ = ufmt::uwriteln!($crate::Out::info(core::concat!("- [", core::file!(), ":", core::line!(), "] - ") ), $($arg)*);
        }
    }
}

#[cfg(any(all(debug_assertions, feature = "level_debug_off"), all(not(debug_assertions), feature = "release_level_debug_off") ) )]
#[macro_export]
///Writes debug log
macro_rules! debug {
    ($($arg:tt)*) => {
    }
}

#[cfg(any(all(debug_assertions, not(feature = "level_debug_off")), all(not(debug_assertions), not(feature = "release_level_debug_off")) ) )]
#[macro_export]
///Writes debug log
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::is_enabled($crate::Level::DEBUG) {
            let _ = ufmt::uwriteln!($crate::Out::debug(core::concat!("- [", core::file!(), ":", core::line!(), "] - ") ), $($arg)*);
        }
    }
}

#[cfg(any(all(debug_assertions, feature = "level_trace_off"), all(not(debug_assertions), feature = "release_level_trace_off") ) )]
#[macro_export]
///Writes trace log
macro_rules! trace {
    ($($arg:tt)*) => {
    }
}

#[cfg(any(all(debug_assertions, not(feature = "level_trace_off")), all(not(debug_assertions), not(feature = "release_level_trace_off")) ) )]
#[macro_export]
///Writes trace log
macro_rules! trace {
    ($($arg:tt)*) => {
        if $crate::is_enabled($crate::Level::TRACE) {
            let _ = ufmt::uwriteln!($crate::Out::trace(core::concat!("- [", core::file!(), ":", core::line!(), "] - ") ), $($arg)*);
        }
    }
}
