use dasm::tier::raw::amd64::*;

fn main() {
	let mut mem: Vec<u8> = vec![];

	let message = b"Hello, world!\n";

	let rax = 0;
	let rdx = 2;
	let rdi = 7; // Argument #1 on linux
	let rsi = 6; // Argument #2 on linux

	// Calls linux sys_write with given string and length
	mem.extend(mov_r64_i64(rdi, 1)); // fd
	mem.extend(mov_r64_i64(rax, 1)); // sys_write
	mem.extend(mov_r64_i64(rsi, message.as_ptr() as _));
	mem.extend(mov_r64_i64(rdx, message.len() as _));
	mem.extend(syscall());
	mem.extend(ret());

	let map = dasm::mmap::Mmap::exec(&mem).expect("Failed to mmap");

	let f: extern "C" fn() = unsafe { std::mem::transmute(map.as_ptr()) };
	f();
}
