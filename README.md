# dasm

This is a tiny assembler that currently supports x86 and amd64.

Support for all instructions is **NOT** planned.
Nor will there be any passes for optimization.

This is simply meant for code generation by JIT compilers and such.

## How it works

There are three tiers to this library,

### Low

This gives you access to the "raw" bytes from an instruction given untagged numeric arguments.
Despite how it sounds, it is pretty nice and easy to use on its own.

**Example**

```rs
	let rax = 0;
	let rcx = 1;
	let rsp = 6; // Argument 2
	let rbp = 7; // Argument 1

	let asm = [
		&tier::raw::amd64::nop() as &[u8],
		&tier::raw::amd64::mov_r64_r64(rax, rbp),
		&tier::raw::amd64::mov_r64_r64(rcx, rsp),
		&tier::raw::amd64::add_r64_r64(rax, rcx),
		&tier::raw::amd64::ret()
	].concat();

	let mut memory = memmap2::MmapMut::map_anon(asm.len())?;
	memory.copy_from_slice(&asm);
	let memory = memory.make_exec()?;

	let adder: extern "C" fn(x: u64, y: u64) -> u64 = unsafe { std::mem::transmute(memory.as_ptr()) };
	println!("{}", adder(5, 200));
```

### Mid (not started)

This level intends to build on `raw` by combining overloads into simple instructions making use of tagged enums.
For example, instead of calling `mov_r64_i64`, you call `mov` with a `Register` and an `Immediate` enum.

### Top (not started)

The final level intends to provide a very small set of instructions that will work across architectures.
For example, an agnostic move function would emit `mov` in x86/amd64 and `li` in RISC-V.

## Why?

- LLVM is too big
- Cranelift is 10x smaller than LLVM, but is very complex for the sake of safety.
- LuaJIT's DynASM involves unnecessary syntax, and Rust is much easier to distribute.

I wanted a simple solution.
