fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Hello from: {}", std::env::consts::ARCH);

	let x0 = 0;
	let x10 = 10;
	let x12 = 12;
	let x17 = 17;
	let a0 = x10;
	let a1 = 11;
	let a7 = x17;
	let a2 = x12;

	let string = b"Hello, RISC-V!\n";

	// unsafe {
	//     std::arch::asm!(
	//         "li a7, 64",        // syscall number for write
	//         "li a0, 1",         // file descriptor (stdout)
	//         "addi a1, {0}, 0",  // load address of the string
	//         "addi a2, {1}, 0",  // length of the string
	//         "ecall",            // make the syscall
	
	//         "li a7, 93",        // syscall number for exit
	//         "li a0, 0",         // exit code
	//         "ecall",            // make the syscall
	
	//         in(reg) string.as_ptr() as usize,
	//         in(reg) string.len()
	//     );
	// };

	let mut mem: Vec<u32> = vec![];
	
	fn add_test(rd: u32, r1: u32, r2: u32) -> u32 {
		(0b0000000 << 25) +
		(r2 << 20) +
		(r1 << 15) +
		(0b000 << 12) +
		(rd << 7) +
		0b0110011
	}
	
	let n1 = 0b0000000_00000_10001_000_00000_0110011;
	let n2 = add_test(0, 0b10001, 0);

	println!("add_example {:032b}", n1);
	println!("add_got {:032b}", n2);
	println!("{}", n1 == n2);

	mem.extend(&[
		dasm::tier::raw::rv32::li(a0, 0),
		dasm::tier::raw::rv32::addi(a0, a0, 55),

		dasm::tier::raw::rv32::ret()
	]);

	// mem.push(dasm::tier::raw::rv32::lui(a0, 0b10000001111110000001_101010101011));
	// mem.push(dasm::tier::raw::rv32::addi(a0, a0, 0b100010010001));
	// mem.push(dasm::tier::raw::rv32::ret());

	let u8_slice: &[u8] = unsafe {
		std::slice::from_raw_parts(
			mem.as_ptr() as *const u8,
			mem.len() * std::mem::size_of::<u32>(),
		)
	};

	let map = dasm::mmap::Mmap::exec(u8_slice)?;
	let f: extern "C" fn() -> u64 = unsafe { std::mem::transmute(map.as_ptr()) };
	println!("{}", f());

	Ok(())
}
