//! `rv32i` implementation
//!
//! Note the lack of `li` - this is because there's no solid way to represent pseudoinstructions that use up multiple instructions.

macro_rules! bitcat {
	( acc($a:expr) $b:literal($e:expr), $($rest:tt)+ ) => {
		(bitcat! { acc($a) $b($e) } | bitcat! { acc($a - $b) $($rest)+ })
	};

	( acc($a:expr) $b:literal($e:expr) ) => {
		($e as u32 & const { (1 << $b) - 1 }) << const { $a - $b }
	};

	( $($rest:tt)+ ) => {
		bitcat! { acc(32) $($rest)+ }
	};
}

pub(crate) use bitcat;

#[inline]
pub fn lui(rd: u8, imm: u32) -> u32 {
	bitcat! { 20(imm), 5(rd), 7(0b0110111) }
}

#[inline]
pub fn auipc(rd: u8, imm: u32) -> u32 {
	bitcat! { 20(imm), 5(rd), 7(0b0010111) }
}

#[inline]
pub fn jal(rd: u8, imm: u32) -> u32 {
	bitcat! { 1(imm >> 20), 10(imm >> 1), 1(imm >> 11), 8(imm >> 12), 5(rd), 7(0b1101111) }
}

#[inline]
pub fn jalr(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b000), 5(rd), 7(0b1100111) }
}

#[inline]
pub fn beq(rs1: u8, rs2: u8, imm: u16) -> u32 {
	bitcat! { 1(imm >> 12), 6(imm >> 5), 5(rs2), 5(rs1), 3(0b000), 4(imm >> 1), 1(imm >> 11), 7(0b1100011) }
}

#[inline]
pub fn bne(rs1: u8, rs2: u8, imm: u16) -> u32 {
	bitcat! { 1(imm >> 12), 6(imm >> 5), 5(rs2), 5(rs1), 3(0b001), 4(imm >> 1), 1(imm >> 11), 7(0b1100011) }
}

#[inline]
pub fn blt(rs1: u8, rs2: u8, imm: u16) -> u32 {
	bitcat! { 1(imm >> 12), 6(imm >> 5), 5(rs2), 5(rs1), 3(0b100), 4(imm >> 1), 1(imm >> 11), 7(0b1100011) }
}

#[inline]
pub fn bge(rs1: u8, rs2: u8, imm: u16) -> u32 {
	bitcat! { 1(imm >> 12), 6(imm >> 5), 5(rs2), 5(rs1), 3(0b101), 4(imm >> 1), 1(imm >> 11), 7(0b1100011) }
}

#[inline]
pub fn bltu(rs1: u8, rs2: u8, imm: u16) -> u32 {
	bitcat! { 1(imm >> 12), 6(imm >> 5), 5(rs2), 5(rs1), 3(0b110), 4(imm >> 1), 1(imm >> 11), 7(0b1100011) }
}

#[inline]
pub fn bgeu(rs1: u8, rs2: u8, imm: u16) -> u32 {
	bitcat! { 1(imm >> 12), 6(imm >> 5), 5(rs2), 5(rs1), 3(0b111), 4(imm >> 1), 1(imm >> 11), 7(0b1100011) }
}

#[inline]
pub fn lb(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b000), 5(rd), 7(0b0000011) }
}

#[inline]
pub fn lh(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b001), 5(rd), 7(0b0000011) }
}

#[inline]
pub fn lw(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b010), 5(rd), 7(0b0000011) }
}

#[inline]
pub fn lbu(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b100), 5(rd), 7(0b0000011) }
}

#[inline]
pub fn lhu(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b101), 5(rd), 7(0b0000011) }
}

#[inline]
pub fn sb(rs1: u8, rs2: u8, imm: u16) -> u32 {
	bitcat! { 7(imm >> 5), 5(rs2), 5(rs1), 3(0b000), 5(imm), 7(0b0100011) }
}

#[inline]
pub fn sh(rs1: u8, rs2: u8, imm: u16) -> u32 {
	bitcat! { 7(imm >> 5), 5(rs2), 5(rs1), 3(0b001), 5(imm), 7(0b0100011) }
}

#[inline]
pub fn sw(rs1: u8, rs2: u8, imm: u16) -> u32 {
	bitcat! { 7(imm >> 5), 5(rs2), 5(rs1), 3(0b010), 5(imm), 7(0b0100011) }
}

#[inline]
pub fn addi(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b000), 5(rd), 7(0b0010011) }
}

#[inline]
pub fn slti(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b010), 5(rd), 7(0b0010011) }
}

#[inline]
pub fn sltiu(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b011), 5(rd), 7(0b0010011) }
}

#[inline]
pub fn xori(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b100), 5(rd), 7(0b0010011) }
}

#[inline]
pub fn ori(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b110), 5(rd), 7(0b0010011) }
}

#[inline]
pub fn andi(rd: u8, rs1: u8, imm: u16) -> u32 {
	bitcat! { 12(imm), 5(rs1), 3(0b111), 5(rd), 7(0b0010011) }
}

#[inline]
pub fn slli(rd: u8, rs1: u8, shamt: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(shamt), 5(rs1), 3(0b001), 5(rd), 7(0b0010011) }
}

#[inline]
pub fn srli(rd: u8, rs1: u8, shamt: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(shamt), 5(rs1), 3(0b101), 5(rd), 7(0b0010011) }
}

#[inline]
pub fn srai(rd: u8, rs1: u8, shamt: u8) -> u32 {
	bitcat! { 7(0b0100000), 5(shamt), 5(rs1), 3(0b101), 5(rd), 7(0b0010011) }
}

#[inline]
pub fn add(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(rs2), 5(rs1), 3(0b000), 5(rd), 7(0b0110011) }
}

#[inline]
pub fn sub(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0100000), 5(rs2), 5(rs1), 3(0b000), 5(rd), 7(0b0110011) }
}

#[inline]
pub fn sll(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(rs2), 5(rs1), 3(0b001), 5(rd), 7(0b0110011) }
}

#[inline]
pub fn slt(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(rs2), 5(rs1), 3(0b010), 5(rd), 7(0b0110011) }
}

#[inline]
pub fn sltu(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(rs2), 5(rs1), 3(0b011), 5(rd), 7(0b0110011) }
}

#[inline]
pub fn xor(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(rs2), 5(rs1), 3(0b100), 5(rd), 7(0b0110011) }
}

#[inline]
pub fn srl(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(rs2), 5(rs1), 3(0b101), 5(rd), 7(0b0110011) }
}

#[inline]
pub fn sra(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0100000), 5(rs2), 5(rs1), 3(0b101), 5(rd), 7(0b0110011) }
}

#[inline]
pub fn or(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(rs2), 5(rs1), 3(0b110), 5(rd), 7(0b0110011) }
}

#[inline]
pub fn and(rd: u8, rs1: u8, rs2: u8) -> u32 {
	bitcat! { 7(0b0000000), 5(rs2), 5(rs1), 3(0b111), 5(rd), 7(0b0110011) }
}

#[inline]
pub fn ecall() -> u32 {
	bitcat! { 12(0b000000000000), 5(0), 3(0b000), 5(0), 7(0b1110011) }
}

#[inline]
pub fn ebreak() -> u32 {
	bitcat! { 12(0b000000000001), 5(0), 3(0b000), 5(0), 7(0b1110011) }
}

#[inline]
pub fn ret() -> u32 {
	jalr(0, 1, 0)
}

#[inline]
pub fn nop() -> u32 {
	addi(0, 0, 0)
}

#[inline]
pub fn mv(rd: u8, rs: u8) -> u32 {
	addi(rd, rs, 0)
}

#[inline]
pub fn j(imm: u32) -> u32 {
	jal(0, imm)
}

#[inline]
pub fn jr(rs: u8) -> u32 {
	jalr(0, rs, 0)
}

#[inline]
pub fn jal_r(rd: u8, rs1: u8) -> u32 {
	jalr(rd, rs1, 0)
}

#[inline]
pub fn call(imm: u32) -> u32 {
	jal(1, imm)
}
