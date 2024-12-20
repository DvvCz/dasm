fn main() -> Result<(), Box<dyn std::error::Error>> {
	use dasm::tier::raw::rv64::*;

	let a0 = 10;
	let a1 = 11;
	let a2 = 12;
	let a7 = 17;

	// Calls linux sys_write with given string and length
	// Note that this implementation stores the string in the same memory, since
	// there's no good way to load a 64 bit immediate address in risc-v.
	let mut mem: Vec<u8> = vec![];
	// .text
	mem.extend(addi(a0, x0, 1).to_le_bytes()); // stdout
	mem.extend(auipc(a1, 0).to_le_bytes()); // a1 = pc
	mem.extend(addi(a1, a1, 24).to_le_bytes()); // string = pc + 24
	mem.extend(addi(a2, x0, 14).to_le_bytes()); // length of string
	mem.extend(addi(a7, x0, 64).to_le_bytes()); // sys_write
	mem.extend(ecall().to_le_bytes());
	mem.extend(ret().to_le_bytes());
	// .data
	mem.extend(b"Hello, world!\n");

	let map = dasm::mmap::Mmap::exec(&mem)?;
	let f: extern "C" fn() = unsafe { std::mem::transmute(map.as_ptr()) };
	f();

	Ok(())
}
