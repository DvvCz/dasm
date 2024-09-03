<h1 align="center"> dasm </h1>

<p align="center">
	A tiny, zero dependency assembler that currently supports x86 and amd64.
</p>

<div align="center">
	<a href="https://crates.io/crates/dasm">
		<img alt="Crates.io Version" src="https://img.shields.io/crates/v/dasm?color=orange">
	</a>
	<a href="https://docs.rs/dasm/latest/dasm">
		<img alt="Docs.rs Link" src="https://img.shields.io/docsrs/dasm?color=blue">
	</a>
	<a href="https://github.com/DvvCz/dasm/actions">
		<img alt="Test Status" src="https://img.shields.io/github/actions/workflow/status/DvvCz/dasm/test.yml?branch=master&label=tests">
	</a>
</div>

## Note

Support for all instructions is **NOT** planned.
Nor will there be any passes for optimization.

This is simply meant for code generation by JIT compilers and such.  
If you want a fully featured library, check out [Iced](https://github.com/icedland/iced)!

## How it works

Code generation doesn't have to be hard. This just provides explicit functions for generating instructions.  
No abstractions for the sake of safety or optimization which add complexity. If you just want to write assembly, this is for you.

**There are three tiers to this library.**

### Low

This gives you access to the "raw" bytes from an instruction given untagged numeric arguments.
Despite how it sounds, it is pretty nice and easy to use on its own.

**Example**

```rust
let rax = 0;
let rsp = 6; // Argument 2
let rbp = 7; // Argument 1

let asm = [
	&dasm::tier::raw::amd64::mov_r64_r64(rax, rbp) as &[u8],
	&dasm::tier::raw::amd64::add_r64_r64(rax, rsp),
	&dasm::tier::raw::amd64::ret()
].concat();

// A helper for making memory executable is included.
let mmapped = dasm::mmap::Mmap::exec(&asm)
	.expect("Failed to mmap");

// Simply cast the bytes to the function you just made.
let adder: extern "C" fn(x: u64, y: u64) -> u64 = unsafe { std::mem::transmute(mmapped.as_ptr()) };
assert_eq!(adder(5, 200), 205);
```

There's also an example showcasing a tiny AOT compiled programming language at [`examples/tinyasm`](https://github.com/DvvCz/dasm/tree/master/examples/tinyasm).

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
