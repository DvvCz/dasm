pub mod tier;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let rax = 0;
	let rcx = 1;
	let rbx = 3;
	let rdi = 5;
	let rsp = 6;
	let rbp = 7;

	let asm = [
		&tier::raw::amd64::nop() as &[u8],
		&tier::raw::amd64::mov_r64_r64(rax, rbp),
		&tier::raw::amd64::mov_r64_r64(rcx, rsp),
		&tier::raw::amd64::add_r64_r64(rax, rcx),
		// &tier::raw::amd64::mov_r64_i64(rcx, 22),
		// &tier::raw::amd64::add_r64_i32(rax, 22),
		// &tier::raw::amd64::mov_r64_r64(rcx, rax),
		// &tier::raw::amd64::push_i32(222),
		// &tier::raw::amd64::nop(),
		// &tier::raw::amd64::pop_r32(rax),
		// &tier::raw::amd64::mov_r64_i64(rax, 0b101),
		// &tier::raw::amd64::mov_r64_i64(rcx, 0b001),
		// &tier::raw::amd64::or_r64_r64(rax, rcx),
		// &tier::raw::amd64::not_r16(rax),
		// &tier::raw::amd64::neg_r64(rax),
		// &tier::raw::amd64::mov_r64_i64(rax, 0) as &[u8],
		// &tier::raw::amd64::add_r16_i16(rax, 22),
		// &tier::raw::amd64::mov_r64_r64(rax, rbp) as &[u8],
		// &tier::raw::amd64::mov_r64_r64(rbx, rsp),
		// &tier::raw::amd64::add_r64_r64(rax, rbx),
		// &tier::raw::amd64::mov_r64_i64(rax, 0),
		// &tier::raw::amd64::add_r64_i32(rax, 55),
		// &tier::raw::amd64::xor_r32_i32(rax, 52),
		&tier::raw::amd64::ret()
	].concat();

	for byte in &asm {
		print!("{:02X} ", byte);
	}

	println!();

	let mut memory = memmap2::MmapMut::map_anon(asm.len())?;
	memory.copy_from_slice(&asm);
	let memory = memory.make_exec()?;

	let f: extern "C" fn(x: u64, y: u64) -> u64 = unsafe { std::mem::transmute(memory.as_ptr()) };
	println!("{}", f(6, 49));

	Ok(())
}
