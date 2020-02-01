pub use ::ufmt::*;

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
