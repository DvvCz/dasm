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

`dasm` has a goal of acting as a lightweight, simple instructions for compilers.

If you want a higher-level library or something more comprehensive, there's other great solutions like [`Iced`](https://github.com/icedland/iced) or [`dynasm-rs`](https://github.com/CensoredUsername/dynasm-rs).

## How it works

Code generation doesn't have to be hard. This just provides explicit functions for generating instructions.  
No abstractions through macros, or some DSL you need to learn. Just functions.

**Example**

```rust
use dasm::tier::raw::amd64::*;

let rax = 0;
let rsi = 6; // Argument 2
let rdi = 7; // Argument 1

let asm = [
	&mov_r64_r64(rax, rdi) as &[u8],
	&add_r64_r64(rax, rsi),
	&ret()
].concat();

// Allocate an executable memory block
let mmapped = dasm::mmap::Mmap::exec(&asm)
	.expect("Failed to mmap");

// Simply cast that memory into a function to call it.
let adder: extern "C" fn(x: u64, y: u64) -> u64 = unsafe { std::mem::transmute(mmapped.as_ptr()) };
assert_eq!(adder(5, 200), 205);
```

*There's also an example showcasing a tiny AOT compiled lisp at [`examples/tinylisp`](https://github.com/DvvCz/dasm/tree/master/examples/tinylisp).*


## Why

All of the other solutions are either too big for my usecase (`LLVM`), too complex (`Cranelift`) or have too many abstractions.  
Sometimes, you don't need all of that - you just need to write some assembly.  
That's what `dasm` is for.

*Of course - No shade to any other library. Anyone can use the tool they prefer.*