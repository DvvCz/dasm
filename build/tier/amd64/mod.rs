use super::x86::util::*;

const COMPAT_16: &str = "COMPAT_16";
const REX_W: &str = "REX_W";

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

		i("push", &[COMPAT_16], &[0x68], Size::U16),
		o("pop", &[COMPAT_16], &[0x58], Size::U16),
		rm("or", &[REX_W], 0x09, Size::U64, Size::U64),
		mi("or", &[REX_W], 0x81, 1, Size::U64, Size::U32),
		rm("xor", &[REX_W], 0x32, Size::U64, Size::U64),
		mi("xor", &[REX_W], 0x81, 6, Size::U64, Size::U32),
		m("not", &[REX_W], 0xF7, 2, Size::U64),
		m("neg", &[REX_W], 0xF7, 3, Size::U64),
		rm("cmp", &[REX_W], 0x3B, Size::U64, Size::U64),
		mi("cmp", &[REX_W], 0x81, 7, Size::U64, Size::U32),
		m("callnai", &[REX_W], 0xFF, 2, Size::U64),
		o("push", &[], &[0x50], Size::U64),
		o("pop", &[], &[0x58], Size::U64),
		zo("syscall", &[], &[0x0F, 0x05])
	].join("\n")
}
