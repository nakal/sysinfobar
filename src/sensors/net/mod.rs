#[cfg(target_os = "freebsd")]
mod bsd;
#[cfg(target_os = "freebsd")]
pub use self::bsd::*;

#[cfg(target_os = "openbsd")]
mod bsd;
#[cfg(target_os = "openbsd")]
pub use self::bsd::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::*;
