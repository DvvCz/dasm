macro_rules! r {
	($name: ident, $funct7: expr, $funct3: expr, $opcode: expr) => {
		#[inline]
		pub const fn $name (rd: u8, rs1: u8, rs2: u8) -> u32 {
			const { assert!($funct3 < 2u32.pow(3)) };
			const { assert!($funct7 < 2u32.pow(7)) };
			const { assert!($opcode < 2u32.pow(7)) };

			($funct7 << 25) +
			(((rs2 & 0b11111) as u32) << 20) +
			(((rs1 & 0b11111) as u32) << 15) +
			($funct3 << 12) +
			(((rd & 0b11111) as u32) << 7) +
			$opcode
		}
	}
}

macro_rules! i {
	($name: ident, $funct3: expr, $opcode: expr) => {
		#[inline]
		pub const fn $name (rd: u8, rs1: u8, imm: u16) -> u32 {
			const { assert!($funct3 < 2u32.pow(3)) };
			const { assert!($opcode < 2u32.pow(7)) };

			(((imm & 0xFFF) as u32) << 20) +
			(((rs1 & 0b11111) as u32) << 15) +
			($funct3 << 12) +
			(((rd & 0b11111) as u32) << 7) +
			$opcode
		}
	}
}

macro_rules! s {
	($name: ident, $funct3: expr, $opcode: expr) => {
		#[inline]
		pub const fn $name (rs1: u8, rs2: u8, imm: u16) -> u32 {
			const { assert!($funct3 < 2u32.pow(3)) };
			const { assert!($opcode < 2u32.pow(7)) };

			(((imm & 0b1111111) as u32) << 25) +
			(((rs2 & 0b11111) as u32) << 20) +
			(((rs1 & 0b11111) as u32) << 15) +
			($funct3 << 12) +
			(((imm & 0b1111) as u32) << 7) +
			$opcode
		}
	}
}

macro_rules! b {
	($name: ident, $funct3: expr, $opcode: expr) => {
		#[inline]
		pub const fn $name (rs1: u8, rs2: u8, imm: u16) -> u32 {
			const { assert!($funct3 < 2u32.pow(3)) };
			const { assert!($opcode < 2u32.pow(7)) };

			((((imm >> 12) & 0b1) as u32) << 31) +
			((((imm >> 5) & 0b111111) as u32) << 25) +
			((rs2 as u32 & 0b11111) << 20) +
			((rs1 as u32 & 0b11111) << 15) +
			($funct3 << 12) +
			((((imm >> 1) & 0b1111) as u32) << 8) +
			((((imm >> 11) & 0b1) as u32) << 7) +
			$opcode
		}
	}
}

macro_rules! u {
	($name: ident, $opcode: expr) => {
		#[inline]
		pub const fn $name(rd: u8, imm: u32) -> u32 {
			const { assert!($opcode < 2u32.pow(7)) };

			((imm & 0xFFFFF000) as u32) +
			((rd as u32 & 0b11111) << 7) +
			$opcode
		}
	}
}

macro_rules! j {
	($name: ident, $opcode: expr) => {
		#[inline]
		pub const fn $name(rd: u8, imm: u32) -> u32 {
			const { assert!($opcode < 2u32.pow(7)) };

			(((imm >> 20) & 0b1) << 31) +
			(((imm >> 1) & 0b1111111111) << 21) +
			(((imm >> 11) & 0b1) << 20) +
			(((imm >> 12) & 0b111111111111) << 12) +
			((rd as u32 & 0b11111) << 7) +
			$opcode
		}
	}
}

r!(add, 0b0000000, 0b000, 0b0110011);
r!(sub, 0b0100000, 0b000, 0b0110011);
r!(sll, 0b0000000, 0b001, 0b0110011);
r!(slt, 0b0000000, 0b010, 0b0110011);
r!(sltu, 0b0000000, 0b011, 0b0110011);
r!(xor, 0b0000000, 0b100, 0b0110011);
r!(srl, 0b0000000, 0b101, 0b0110011);
r!(sra, 0b0100000, 0b101, 0b0110011);
r!(or, 0b0000000, 0b110, 0b0110011);
r!(and, 0b0000000, 0b111, 0b0110011);

i!(lb, 0b000, 0b0000011);
i!(lh, 0b001, 0b0000011);
i!(lw, 0b010, 0b0000011);
i!(lbu, 0b100, 0b0000011);
i!(lhu, 0b101, 0b0000011);
i!(addi, 0b000, 0b0010011);
i!(slti, 0b010, 0b0010011);
i!(sltiu, 0b011, 0b0010011);
i!(xori, 0b100, 0b0010011);
i!(ori, 0b110, 0b0010011);
i!(andi, 0b111, 0b0010011);
i!(slli, 0b001, 0b0010011);
i!(srli, 0b101, 0b0010011);
i!(srai, 0b101, 0b0010011);
i!(jalr, 0b000, 0b1100111);

s!(sb, 0b000, 0b0100011);
s!(sh, 0b001, 0b0100011);
s!(sw, 0b010, 0b0100011);

b!(beq, 0b000, 0b1100011);
b!(bne, 0b001, 0b1100011);
b!(blt, 0b100, 0b1100011);
b!(bge, 0b101, 0b1100011);
b!(bltu, 0b110, 0b1100011);
b!(bgeu, 0b111, 0b1100011);

// u!(lui, 0b0110111);

#[inline]
pub fn lui(rd: u8, imm: u32) -> u32 {
	(((imm & 0xFFFFF) as u32) << 12) +
	((rd as u32 & 0b11111) << 7) +
	0b0110111
}

u!(auipc, 0b0010111);

j!(jal, 0b1101111);

#[inline]
pub fn nop() -> u32 {
	addi(0, 0, 0)
}

#[inline]
pub fn li(rd: u8, imm: u16) -> u32 {
	addi(rd, 0, imm)
}

#[inline]
pub fn li_32(rd: u8, imm: u32) -> [u32; 2] {
	[
		lui(rd, (imm >> 12) as _),
		addi(rd, rd, (imm & 0xFFF) as _)
	]
}

pub fn li_64(rd: u8, imm: u64) -> [u32; 8] {
	[
		lui(rd, (imm >> 44) as _), // Load upper 20 bits

        addi(rd, rd, ((imm >> 32) & 0xFFF) as _), // Add next 12 bits
        slli(rd, rd, 12), // Shift left by 12 bits

        addi(rd, rd, ((imm >> 20) & 0xFFF) as _), // Add next 12 bits
        slli(rd, rd, 12), // Shift left by 12 bits

        addi(rd, rd, ((imm >> 8) & 0xFFF) as _), // Add next 12 bits
        slli(rd, rd, 12), // Shift left by 12 bits

        addi(rd, rd, (imm & 0xFF) as _), // Add final 8 bits
	]
}

// pub fn li_64(rd: u8, imm: u64) -> [u32; 5] {
// 	let [l1, l2] = li_32(rd, (imm >> 32) as _);
// 	let [l3, l4] = li_32(rd, (imm & 0b11111111111111111111111111111111) as _);

// 	[
// 		l1,
// 		l2,
// 		slli(rd, rd, 32),
// 		l3,
// 		l4
// 	]
// }

#[inline]
pub fn mv(rd: u8, rs: u8) -> u32 {
	addi(rd, rs, 0)
}

#[inline]
pub fn not(rd: u8, rs: u8) -> u32 {
	xori(rd, rs, 0xFFF)
}

#[inline]
pub fn neg(rd: u8, rs: u8) -> u32 {
	sub(rd, 0, rs)
}

#[inline]
pub fn seqz(rd: u8, rs: u8) -> u32 {
	sltiu(rd, rs, 1)
}

#[inline]
pub fn snez(rd: u8, rs: u8) -> u32 {
	sltu(rd, 0, rs)
}

#[inline]
pub fn sltz(rd: u8, rs: u8) -> u32 {
	slt(rd, rs, 0)
}

#[inline]
pub fn sgtz(rd: u8, rs: u8) -> u32 {
	slt(rd, 0, rs)
}

#[inline]
pub fn beqz(rs: u8, offset: u16) -> u32 {
	beq(rs, 0, offset)
}

#[inline]
pub fn bnez(rs: u8, offset: u16) -> u32 {
	bne(rs, 0, offset)
}

#[inline]
pub fn j(offset: u32) -> u32 {
	jal(0, offset)
}

#[inline]
pub fn jr(rs: u8) -> u32 {
	jalr(0, rs, 0)
}

#[inline]
pub fn ret() -> u32 {
	jr(1)
}

#[inline]
pub fn la(rd: u8, label: u32) -> [u32; 2] {
    [auipc(rd, label >> 12), addi(rd, rd, (label & 0xFFF) as _)]
}

#[inline]
pub fn ecall() -> u32 {
	0b000000000000_00000_000_00000_1110011
}

#[inline]
pub fn ebreak() -> u32 {
	0b000000000001_00000_000_00000_1110011
}
