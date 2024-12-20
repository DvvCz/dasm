//! `rv64i` implementation
//!
//! Note the lack of `li` - this is because there's no solid way to represent pseudoinstructions that use up multiple instructions.
//! This includes all of the instructions from 32 bit RISC-V, which RV64 is compatible with.

pub use crate::tier::raw::rv32::*;

#[inline]
pub fn lwu(rd: u8, rs1: u8, imm: i32) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b110), 5(rd), 7(0b0000011) }
}

#[inline]
pub fn ld(rd: u8, rs1: u8, imm: i32) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b011), 5(rd), 7(0b0000011) }
}

#[inline]
pub fn sd(rs1: u8, rs2: u8, imm: i32) -> u32 {
	bitcat! { 7(imm >> 5), 5(rs2), 5(rs1), 3(0b011), 5(imm), 7(0b0100011) }
}

#[inline]
pub fn addiw(rd: u8, rs1: u8, imm: i32) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b000), 5(rd), 7(0b0011011) }
}

#[inline]
pub fn slliw(rd: u8, rs1: u8, shamt: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(shamt), 5(rs1), 3(0b001), 5(rd), 7(0b0011011) }
}

#[inline]
pub fn srliw(rd: u8, rs1: u8, shamt: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(shamt), 5(rs1), 3(0b101), 5(rd), 7(0b0011011) }
}

#[inline]
pub fn sraiw(rd: u8, rs1: u8, shamt: u8) -> u32 {
	bitcat! { 7(0b0100000), 5(shamt), 5(rs1), 3(0b101), 5(rd), 7(0b0011011) }
}

#[inline]
pub fn addw(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(rs2), 5(rs1), 3(0b000), 5(rd), 7(0b0111011) }
}

#[inline]
pub fn subw(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0100000), 5(rs2), 5(rs1), 3(0b000), 5(rd), 7(0b0111011) }
}

#[inline]
pub fn sllw(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(rs2), 5(rs1), 3(0b001), 5(rd), 7(0b0111011) }
}

#[inline]
pub fn srlw(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(rs2), 5(rs1), 3(0b101), 5(rd), 7(0b0111011) }
}

#[inline]
pub fn sraw(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0100000), 5(rs2), 5(rs1), 3(0b101), 5(rd), 7(0b0111011) }
}

#[inline]
pub fn slli(rd: u8, rs1: u8, shamt: u8) -> u32 {
	bitcat! { 6(0b000000), 6(shamt), 5(rs1), 3(0b001), 5(rd), 7(0b0010011) }
}

#[inline]
pub fn srli(rd: u8, rs1: u8, shamt: u8) -> u32 {
	bitcat! { 6(0b000000), 6(shamt), 5(rs1), 3(0b101), 5(rd), 7(0b0010011) }
}

#[inline]
pub fn srai(rd: u8, rs1: u8, shamt: u8) -> u32 {
	bitcat! { 6(0b010000), 6(shamt), 5(rs1), 3(0b101), 5(rd), 7(0b0010011) }
}
