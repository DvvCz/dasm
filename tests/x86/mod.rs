const EAX: u8 = 0;
const ECX: u8 = 1;
const EBX: u8 = 3;
const EDI: u8 = 5;
const ESP: u8 = 6;
const EBP: u8 = 7;

#[test]
fn test_adder() {
	let adder = dasm::mmap::Mmap::exec([
		&dasm::tier::raw::x86::mov_r32_r32(EAX, EBP) as &[u8],
		&dasm::tier::raw::x86::mov_r32_r32(ECX, ESP),
		&dasm::tier::raw::x86::add_r32_r32(EAX, ECX),
		&dasm::tier::raw::x86::ret()
	].concat()).unwrap();

	let adder: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(adder.as_ptr()) };

	assert_eq!(adder(64, 64), 128);
	assert_eq!(adder(0, 0), 0);
	assert_eq!(adder((-1i64) as u64, 2) as i64, 1);
}
