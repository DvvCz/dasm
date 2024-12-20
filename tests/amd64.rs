#![cfg(target_arch = "x86_64")]

const RAX: u8 = 0;
const RCX: u8 = 1;
const RDX: u8 = 2;
const RSI: u8 = 6; // Arg #2
const RDI: u8 = 7; // Arg #1

#[test]
#[cfg(target_os = "linux")]
fn test_print() {
	use dasm::tier::raw::amd64::*;

	let message = b"Hello, world!\n";

	let map = dasm::mmap::Mmap::exec(&[
		&mov_r64_i64(RDI, 1) as &[u8],
		&mov_r64_i64(RAX, 1),
		&mov_r64_i64(RSI, message.as_ptr() as _),
		&mov_r64_i64(RDX, message.len() as _),
		&syscall(),

		&ret()
	].concat()).expect("Failed to mmap");

	let f: extern "C" fn() = unsafe { std::mem::transmute(map.as_ptr()) };
	f();

	// TODO: Verify output by intercepting it
}

#[test]
fn test_raw_adder() {
	use dasm::tier::raw::amd64::*;

	let adder = dasm::mmap::Mmap::exec([
		&mov_r64_r64(RAX, RDI) as &[u8],
		&mov_r64_r64(RCX, RSI),
		&add_r64_r64(RAX, RCX),
		&ret()
	].concat()).unwrap();

	let adder: extern "C" fn(u64, u64) -> u64 = unsafe { std::mem::transmute(adder.as_ptr()) };

	assert_eq!(adder(64, 64), 128);
	assert_eq!(adder(0, 0), 0);
	assert_eq!(adder((-1i64) as u64, 2) as i64, 1);
}
