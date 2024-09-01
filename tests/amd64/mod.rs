const RAX: u8 = 0;
const RCX: u8 = 1;
const RBX: u8 = 3;
const RDI: u8 = 5;
const RSP: u8 = 6;
const RBP: u8 = 7;

#[test]
fn test_adder() {
	let adder = dasm::mmap::Mmap::exec([
		&dasm::tier::raw::amd64::mov_r64_r64(RAX, RBP) as &[u8],
		&dasm::tier::raw::amd64::mov_r64_r64(RCX, RSP),
		&dasm::tier::raw::amd64::add_r64_r64(RAX, RCX),
		&dasm::tier::raw::amd64::ret()
	].concat()).unwrap();

	let adder: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(adder.as_ptr()) };

	assert_eq!(adder(64, 64), 128);
	assert_eq!(adder(0, 0), 0);
	assert_eq!(adder((-1i64) as u64, 2) as i64, 1);
}
