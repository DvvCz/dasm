pub mod util;
use util::*;

pub fn src_amd64_compatible() -> String {
	[
		m("not", &[], 0xF6, 2, Size::U8),
		m("not", &[], 0xF7, 2, Size::U16),
		m("not", &[], 0xF7, 2, Size::U32),
		rm("xor", &[], 0x30, Size::U8, Size::U8),
		rm("xor", &[], 0x31, Size::U16, Size::U16),
		rm("xor", &[], 0x31, Size::U32, Size::U32),
		mi("xor", &[], 0x81, 6, Size::U32, Size::U32),
		zo("nop", &[], &[0x90]),
		zo("ret", &[], &[0xC3]),
		zo("leave", &[], &[0xC9]),
		i("push", &[], &[0x68], Size::U8),
		i("push", &[], &[0x68], Size::U16),
		i("push", &[], &[0x68], Size::U32),
		o("push", &[], &[0x50], Size::U16),
		o("pop", &[], &[0x58], Size::U16),
		i("int", &[], &[0xCD], Size::U8),
		zo("int1", &[], &[0xF1]),
		zo("int3", &[], &[0xCC]),
		m("neg", &[], 0xF7, 3, Size::U16),
		m("neg", &[], 0xF7, 3, Size::U32),
		rm("add", &[], 0x03, Size::U16, Size::U16),
		rm("add", &[], 0x03, Size::U32, Size::U32),
		mi("add", &[], 0x81, 0, Size::U16, Size::U16),
		mi("add", &[], 0x81, 0, Size::U32, Size::U32),
		rm("sub", &[], 0x2B, Size::U16, Size::U16),
		rm("sub", &[], 0x2B, Size::U32, Size::U32),
		mi("sub", &[], 0x81, 5, Size::U16, Size::U16),
		mi("sub", &[], 0x81, 5, Size::U32, Size::U32),
		m("mul", &[], 0xF6, 4, Size::U8),
		m("mul", &[], 0xF7, 4, Size::U16),
		m("mul", &[], 0xF7, 4, Size::U32),
		m("div", &[], 0xF6, 6, Size::U8),
		m("div", &[], 0xF7, 6, Size::U16),
		m("div", &[], 0xF7, 6, Size::U32),
		rm("cmp", &[], 0x3B, Size::U16, Size::U16),
		rm("cmp", &[], 0x3B, Size::U32, Size::U32),
		mi("cmp", &[], 0x80, 7, Size::U8, Size::U8),
		mi("cmp", &[], 0x81, 7, Size::U16, Size::U16),
		mi("cmp", &[], 0x81, 7, Size::U32, Size::U32),
		d("callnrd", &[], &[0xE8], Size::U16),
		d("callnrd", &[], &[0xE8], Size::U32),
		m("callnai", &[], 0xFF, 2, Size::U16),
		m("callnai", &[], 0xFF, 2, Size::U32),
		rm("mov", &[], 0x8A, Size::U8, Size::U8),
		rm("mov", &[], 0x8B, Size::U16, Size::U16),
		rm("mov", &[], 0x8B, Size::U32, Size::U32),
		oi("mov", &[], 0xB0, Size::U8, Size::U8),
		oi("mov", &[], 0xB8, Size::U16, Size::U16),
		oi("mov", &[], 0xB8, Size::U32, Size::U32)
	].join("\n")
}

#[rustfmt::skip]
pub fn src_x86_only() -> String {
	[
		zo("into", &[], &[0xCE]),
		o("push", &[], &[0x50], Size::U32),
		o("pop", &[], &[0x58], Size::U32)
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
