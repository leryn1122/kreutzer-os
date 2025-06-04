#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[allow(unused_imports)]
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

pub mod shared;
#[allow(unused_imports)]
pub use self::shared::*;
