#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
///Alias to platform logger
pub type Out = wasm::Console;

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
///Alias to platform logger
pub type Out = android::Log;

#[cfg(not(any(all(target_arch = "wasm32", target_os = "unknown"), target_os = "android")))]
mod std_c;
#[cfg(not(any(all(target_arch = "wasm32", target_os = "unknown"), target_os = "android")))]
///Alias to platform logger
pub type Out = std_c::FdWriter;

#[cfg(all(not(target_arch = "wasm32"), target_os = "unknown"))]
mod noop;
#[cfg(all(not(target_arch = "wasm32"), target_os = "unknown"))]
///Alias to noop logger
pub type Out = noop::Noop;
