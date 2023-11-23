//! Safe abstractions over WDF APIs

#[cfg(feature = "wdf")]
mod spinlock;
#[cfg(feature = "wdf")]
mod timer;

#[cfg(feature = "wdf")]
pub use spinlock::*;
#[cfg(feature = "wdf")]
pub use timer::*;
