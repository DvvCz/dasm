pub(crate) const MODRM_INDIRECT: u8 = 0b00;
pub(crate) const MODRM_DIRECT: u8 = 0b11;

#[inline]
pub(crate) const fn mod_rm(mode: u8, src: u8, dst: u8) -> u8 {
	(mode << 6) | ((src & 0b111) << 3) | (dst & 0b111)
}

include!(concat!(env!("OUT_DIR"), "/x86.rs"));
