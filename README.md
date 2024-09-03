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

Support for every single instruction is **NOT** planned.
Nor will there be any passes for optimization.

This has a simple goal of assembling instructions for compilers.
If you want a fully featured library, please refer to [`Iced`](https://github.com/icedland/iced)

## How it works

Code generation doesn't have to be hard. This just provides explicit functions for generating instructions.  
No abstractions for the sake of safety or optimization which add complexity. If you just want to write assembly, this is for you.

**This library has tiered abstraction levels.**

### Raw

This gives you access to the "raw" bytes from an instruction given untagged numeric arguments.
Despite how it sounds, it is pretty nice to use on its own.

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

There's also an example showcasing a tiny AOT compiled lisp at [`examples/tinylisp`](https://github.com/DvvCz/dasm/tree/master/examples/tinylisp).

## Other Tiers

At the moment, other tiers are not implemented as I plan out how these abstractions would go.

Hopefully they'd involve abstracting away overloads with tagged enums and cross architecture compatibility.

## Why

I felt the need for an assembler.

LLVM was out of the question due to its size and difficult integration to Rust.

I struggled to work with Cranelift due to its complexity for the sake of safety and performance.

But sometimes, you don't care about safety or performance. You just want a simple solution to get it done.

That's what `dasm` is supposed to be.
