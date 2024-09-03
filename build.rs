enum Size {
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

mod x86 {
	use super::{unpack, Size};

	pub fn oi(inst: &'static str, op: u8, rdst: Size, isrc: Size) -> String {
		let total_bytes = 1 + isrc.bytes();
		let unpack = unpack(isrc.bytes());

		indoc::formatdoc! {"
			#[inline]
			pub const fn {inst}_r{rdst}_i{isrc}(dst: u8, src: u{isrc}) -> [u8; {total_bytes}] {{
				let b = src.to_le_bytes();
				[0x{op:02X} + dst, {unpack}]
			}}
		"}
	}

	pub fn mi(inst: &str, op: u8, code: u8, rdst: Size, isrc: Size) -> String {
		let total_bytes = 2 + isrc.bytes();
		let unpack = unpack(isrc.bytes());

		indoc::formatdoc! {"
			#[inline]
			pub const fn {inst}_r{rdst}_i{isrc}(dst: u8, src: u{isrc}) -> [u8; {total_bytes}] {{
				let b = src.to_le_bytes();
				[0x{op:02X}, mod_rm(MODRM_DIRECT, {code}, dst), {unpack}]
			}}
		"}
	}

	#[inline]
	pub fn rm(inst: &str, op: u8, rdst: Size, rsrc: Size) -> String {
		indoc::formatdoc! {"
			#[inline]
			pub const fn {inst}_r{rdst}_r{rsrc}(dst: u8, src: u8) -> [u8; 2] {{
				[0x{op:02X}, mod_rm(MODRM_DIRECT, dst, src)]
			}}
		"}
	}

	#[inline]
	pub fn m(inst: &str, op: u8, code: u8, rdst: Size) -> String {
		indoc::formatdoc! {"
			#[inline]
			pub const fn {inst}_r{rdst}(dst: u8) -> [u8; 2] {{
				[0x{op:02X}, mod_rm(MODRM_DIRECT, {code}, dst)]
			}}
		"}
	}

	pub fn zo(inst: &str, op: u8) -> String {
		indoc::formatdoc! {"
			#[inline]
			pub const fn {inst}() -> [u8; 1] {{
				[0x{op:02X}]
			}}
		"}
	}

	pub fn i(inst: &str, op: u8, src: Size) -> String {
		let total_bytes = 1 + src.bytes();
		let unpack = unpack(src.bytes());

		indoc::formatdoc! {"
			#[inline]
			pub const fn {inst}_i{src}(src: u{src}) -> [u8; {total_bytes}] {{
				let b = src.to_le_bytes();
				[0x{op:02X}, {unpack}]
			}}
		"}
	}

	// This encodes the same as an immediate. A separate function purely for distinction.
	pub use i as d;

	pub fn o(inst: &'static str, op: u8, rdst: Size) -> String {
		indoc::formatdoc! {"
			#[inline]
			pub const fn {inst}_r{rdst}(dst: u8) -> [u8; 1] {{
				[0x{op:02X} + dst]
			}}
		"}
	}

	pub fn src_amd64_compatible() -> String {
		[
			m("not", 0xF6, 2, Size::U8),
			m("not", 0xF7, 2, Size::U16),
			m("not", 0xF7, 2, Size::U32),

			rm("xor", 0x30, Size::U8, Size::U8),
			rm("xor", 0x31, Size::U16, Size::U16),
			rm("xor", 0x31, Size::U32, Size::U32),

			mi("xor", 0x81, 6, Size::U32, Size::U32),

			zo("nop", 0x90),
			zo("ret", 0xC3),
			zo("leave", 0xC9),

			i("push", 0x68, Size::U8),
			i("push", 0x68, Size::U16),
			i("push", 0x68, Size::U32),

			o("push", 0x50, Size::U16),
			o("pop", 0x58, Size::U16),

			i("int", 0xCD, Size::U8),
			zo("int0", 0xCE),
			zo("int1", 0xF1),
			zo("int3", 0xCC),

			m("neg", 0xF7, 3, Size::U16),
			m("neg", 0xF7, 3, Size::U32),

			rm("add", 0x03, Size::U16, Size::U16),
			rm("add", 0x03, Size::U32, Size::U32),
			mi("add", 0x81, 0, Size::U16, Size::U16),
			mi("add", 0x81, 0, Size::U32, Size::U32),

			rm("sub", 0x2B, Size::U16, Size::U16),
			rm("sub", 0x2B, Size::U32, Size::U32),
			mi("sub", 0x81, 5, Size::U16, Size::U16),
			mi("sub", 0x81, 5, Size::U32, Size::U32),

			m("mul", 0xF6, 4, Size::U8),
			m("mul", 0xF7, 4, Size::U16),
			m("mul", 0xF7, 4, Size::U32),

			m("div", 0xF6, 6, Size::U8),
			m("div", 0xF7, 6, Size::U16),
			m("div", 0xF7, 6, Size::U32),

			rm("cmp", 0x3B, Size::U16, Size::U16),
			rm("cmp", 0x3B, Size::U32, Size::U32),

			mi("cmp", 0x80, 7, Size::U8, Size::U8),
			mi("cmp", 0x81, 7, Size::U16, Size::U16),
			mi("cmp", 0x81, 7, Size::U32, Size::U32),

			d("callnrd", 0xE8, Size::U16),
			d("callnrd", 0xE8, Size::U32),

			m("callnai", 0xFF, 2, Size::U16),
			m("callnai", 0xFF, 2, Size::U32),

			rm("mov", 0x8A, Size::U8, Size::U8),
			rm("mov", 0x8B, Size::U16, Size::U16),
			rm("mov", 0x8B, Size::U32, Size::U32),

			oi("mov", 0xB0, Size::U8, Size::U8),
			oi("mov", 0xB8, Size::U16, Size::U16),
			oi("mov", 0xB8, Size::U32, Size::U32),
		].join("\n")
	}

	pub fn src_x86_only() -> String {
		[
			o("push", 0x50, Size::U32),
			o("pop", 0x58, Size::U32),
		].join("\n")
	}

	pub fn src() -> String {
		let amd64_compatible = src_amd64_compatible();
		let x86_only = src_x86_only();

		indoc::formatdoc! {"
			pub(crate) mod compatible {{
				pub(crate) use super::prelude::*;
				{amd64_compatible}
			}}

			pub use compatible::*;

			{x86_only}
		"}
	}
}

mod amd64 {
	use super::{unpack, Size};

	const COMPAT_16: &str = "COMPAT_16";
	const REX_W: &str = "REX_W";

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

	#[inline]
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

	#[inline]
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

	pub fn zo(inst: &str, prefixes: &[&str], op: u8) -> String {
		let total_bytes = prefixes.len() + 1;
		let prefixes = prefixes.iter().map(|s| format!("{s}, ")).collect::<Vec<_>>().concat();

		indoc::formatdoc! {"
			#[inline]
			pub const fn {inst}() -> [u8; {total_bytes}] {{
				[{prefixes}0x{op:02X}]
			}}
		"}
	}

	pub fn i(inst: &str, prefixes: &[&str], op: u8, src: Size) -> String {
		let total_bytes = prefixes.len() as u8 + 1 + src.bytes();
		let unpack = unpack(src.bytes());
		let prefixes = prefixes.iter().map(|s| format!("{s}, ")).collect::<Vec<_>>().concat();

		indoc::formatdoc! {"
			#[inline]
			pub const fn {inst}_i{src}(src: u{src}) -> [u8; {total_bytes}] {{
				let b = src.to_le_bytes();
				[{prefixes}0x{op:02X}, {unpack}]
			}}
		"}
	}

	// This encodes the same as an immediate. A separate function purely for distinction.
	pub use i as d;

	pub fn o(inst: &str, prefixes: &[&str], op: u8, rdst: Size) -> String {
		let total_bytes = prefixes.len() + 1;
		let prefixes = prefixes.iter().map(|s| format!("{s}, ")).collect::<Vec<_>>().concat();

		indoc::formatdoc! {"
			#[inline]
			pub const fn {inst}_r{rdst}(dst: u8) -> [u8; {total_bytes}] {{
				[{prefixes}0x{op:02X} + dst]
			}}
		"}
	}

	pub fn src() -> String {
		[
			rm("add", &[COMPAT_16], 0x03, Size::U16, Size::U16),
			rm("add", &[REX_W], 0x03, Size::U64, Size::U64),
			mi("add", &[COMPAT_16], 0x81, 0, Size::U16, Size::U16),
			mi("add", &[REX_W], 0x81, 0, Size::U64, Size::U32),

			rm("sub", &[COMPAT_16], 0x2B, Size::U16, Size::U16),
			rm("sub", &[REX_W], 0x2B, Size::U64, Size::U64),
			mi("sub", &[COMPAT_16], 0x81, 5, Size::U16, Size::U16),
			mi("sub", &[REX_W], 0x81, 5, Size::U64, Size::U32),

			m("mul", &[REX_W], 0xF7, 4, Size::U64),
			m("div", &[REX_W], 0xF7, 6, Size::U64),

			rm("mov", &[REX_W], 0x8B, Size::U64, Size::U64),
			oi("mov", &[REX_W], 0xB8, Size::U64, Size::U64),

			i("push", &[COMPAT_16], 0x68, Size::U16),
			o("pop", &[COMPAT_16], 0x58, Size::U16),

			rm("or", &[REX_W], 0x09, Size::U64, Size::U64),
			mi("or", &[REX_W], 0x81, 1, Size::U64, Size::U32),

			rm("xor", &[REX_W], 0x32, Size::U64, Size::U64),
			mi("xor", &[REX_W], 0x81, 6, Size::U64, Size::U32),

			m("not", &[REX_W], 0xF7, 2, Size::U64),
			m("neg", &[REX_W], 0xF7, 3, Size::U64),

			rm("cmp", &[REX_W], 0x3B, Size::U64, Size::U64),
			mi("cmp", &[REX_W], 0x81, 7, Size::U64, Size::U32),

			m("callnai", &[REX_W], 0xFF, 2, Size::U64),

			o("push", &[], 0x50, Size::U64),
			o("pop", &[], 0x58, Size::U64)
		].join("\n")
	}
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("cargo:rerun-if-changed=build.rs");

	let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

	std::fs::write(out_path.join("x86.rs"), x86::src())?;
	std::fs::write(out_path.join("amd64.rs"), amd64::src())?;

	Ok(())
}
