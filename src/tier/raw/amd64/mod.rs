// AMD64 should support all? of these
pub use crate::tier::raw::x86::*;

pub const REX: u8 = 0b0100_0000;

/// 0b0100_1000
pub const REX_W: u8 = REX + 0b1000;

/// Override 32-bit default for compatibility with 16 bit functions on amd64.
pub const COMPAT_16: u8 = 0x66;

include!(concat!(env!("OUT_DIR"), "/amd64.rs"));
