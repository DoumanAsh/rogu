# rogu

[![Crates.io](https://img.shields.io/crates/v/rogu.svg)](https://crates.io/crates/rogu)
[![Documentation](https://docs.rs/rogu/badge.svg)](https://docs.rs/crate/rogu/)
[![Build](https://github.com/DoumanAsh/rogu/workflows/Rust/badge.svg)](https://github.com/DoumanAsh/rogu/actions?query=workflow%3ARust)

Logging library, plain and simple.

Simple logger.

Minimal customization, builtin loggers:

- Web - Uses console API;
- Android Log;
- C stdio - writes to stdout/stderr;

## Features

Note: all features must be used in binary code, not in library

#### Loggers

- `ufmt` - Enables [ufmt](https://github.com/japaric/ufmt) traits instead of core's `fmt`.
- `std` - Enables `std` usage, adding ability to use timestamps (not used in Web and Android)
- `log` - Enables `log` usage, adding `log` logs to the output.

#### Compile time macros

- `level_error_off` - Disables `error!` in debug mode;
- `level_warn_off` - Disables `warn!` in debug mode;
- `level_info_off` - Disables `info!` in debug mode;
- `level_debug_off` - Disables `debug!` in debug mode;
- `level_trace_off` - Disables `trace!` in debug mode;
- `level_all_off`- Disables all macros in debug mode;

- `release_level_error_off` - Disables `error!` in release mode;
- `release_level_warn_off` - Disables `warn!` in release mode;
- `release_level_info_off` - Disables `info!` in release mode;
- `release_level_debug_off` - Disables `debug!` in release mode;
- `release_level_trace_off` - Disables `trace!` in release mode;
- `release_level_all_off`- Disables all macros in release mode;
