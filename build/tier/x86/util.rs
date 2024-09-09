pub enum Size {
	U64,
	U32,
	U16,
	U8
}

impl std::fmt::Display for Size {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Self::U64 => f.write_str("64"),
			Self::U32 => f.write_str("32"),
			Self::U16 => f.write_str("16"),
			Self::U8 => f.write_str("8"),
		}
	}
}

impl Size {
	pub fn bytes(&self) -> u8 {
		match *self {
			Self::U64 => 8,
			Self::U32 => 4,
			Self::U16 => 2,
			Self::U8 => 1,
		}
	}
}

pub fn unpack(amt: u8) -> String {
	(0..amt)
		.map(|i| format!("b[{i}]"))
		.collect::<Vec<_>>()
		.join(", ")
}

pub fn oi(inst: &str, prefixes: &[&str], op: u8, rdst: Size, isrc: Size) -> String {
	let total_bytes = prefixes.len() as u8 + 1 + isrc.bytes();
	let unpack = unpack(isrc.bytes());
	let prefixes = prefixes.iter().map(|s| format!("{s}, ")).collect::<Vec<_>>().concat();

	indoc::formatdoc! {"
		#[inline]
		pub const fn {inst}_r{rdst}_i{isrc}(dst: u8, src: u{isrc}) -> [u8; {total_bytes}] {{
			let b = src.to_le_bytes();
			[{prefixes}0x{op:02X} + dst, {unpack}]
		}}
	"}
}

pub fn mi(inst: &str, prefixes: &[&str], op: u8, code: u8, rdst: Size, isrc: Size) -> String {
	let total_bytes = prefixes.len() as u8 + 2 + isrc.bytes();
	let unpack = unpack(isrc.bytes());
	let prefixes = prefixes.iter().map(|s| format!("{s}, ")).collect::<Vec<_>>().concat();

	indoc::formatdoc! {"
		#[inline]
		pub const fn {inst}_r{rdst}_i{isrc}(dst: u8, src: u{isrc}) -> [u8; {total_bytes}] {{
			let b = src.to_le_bytes();
			[{prefixes}0x{op:02X}, mod_rm(MODRM_DIRECT, {code}, dst), {unpack}]
		}}
	"}
}

pub fn rm(inst: &str, prefixes: &[&str], op: u8, rdst: Size, rsrc: Size) -> String {
	let total_bytes = prefixes.len() + 2;
	let prefixes = prefixes.iter().map(|s| format!("{s}, ")).collect::<Vec<_>>().concat();

	indoc::formatdoc! {"
		#[inline]
		pub const fn {inst}_r{rdst}_r{rsrc}(dst: u8, src: u8) -> [u8; {total_bytes}] {{
			[{prefixes}0x{op:02X}, mod_rm(MODRM_DIRECT, dst, src)]
		}}
	"}
}

pub fn m(inst: &str, prefixes: &[&str], op: u8, code: u8, rdst: Size) -> String {
	let total_bytes = prefixes.len() + 2;
	let prefixes = prefixes.iter().map(|s| format!("{s}, ")).collect::<Vec<_>>().concat();

	indoc::formatdoc! {"
		#[inline]
		pub const fn {inst}_r{rdst}(dst: u8) -> [u8; {total_bytes}] {{
			[{prefixes}0x{op:02X}, mod_rm(MODRM_DIRECT, {code}, dst)]
		}}
	"}
}

pub fn zo(inst: &str, prefixes: &[&str], ops: &[u8]) -> String {
	let total_bytes = prefixes.len() + ops.len();
	let prefixes = prefixes.iter().map(|s| format!("{s}, ")).collect::<Vec<_>>().concat();
	let ops = ops.iter().map(|op| format!("0x{op:02X}")).collect::<Vec<_>>().join(", ");

	indoc::formatdoc! {"
		#[inline]
		pub const fn {inst}() -> [u8; {total_bytes}] {{
			[{prefixes}{ops}]
		}}
	"}
}

pub fn i(inst: &str, prefixes: &[&str], ops: &[u8], src: Size) -> String {
	let total_bytes = prefixes.len() + ops.len() + src.bytes() as usize;
	let unpack = unpack(src.bytes());
	let prefixes = prefixes.iter().map(|s| format!("{s}, ")).collect::<Vec<_>>().concat();
	let ops = ops.iter().map(|op| format!("0x{op:02X}")).collect::<Vec<_>>().join(", ");

	indoc::formatdoc! {"
		#[inline]
		pub const fn {inst}_i{src}(src: u{src}) -> [u8; {total_bytes}] {{
			let b = src.to_le_bytes();
			[{prefixes}{ops}, {unpack}]
		}}
	"}
}

// This encodes the same as an immediate. A separate function purely for distinction.
pub use i as d;

pub fn o(inst: &str, prefixes: &[&str], ops: &[u8], rdst: Size) -> String {
	let total_bytes = prefixes.len() + ops.len();
	let prefixes = prefixes.iter().map(|s| format!("{s}, ")).collect::<Vec<_>>().concat();
	let ops = ops.iter().map(|op| format!("0x{op:02X}")).collect::<Vec<_>>().join(", ");

	indoc::formatdoc! {"
		#[inline]
		pub const fn {inst}_r{rdst}(dst: u8) -> [u8; {total_bytes}] {{
			[{prefixes}{ops} + dst]
		}}
	"}
}
