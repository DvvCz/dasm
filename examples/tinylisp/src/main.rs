#[derive(Debug, Clone)]
enum Token<'a> {
	LParen,
	RParen,
	Ident(&'a [u8]),
	String(std::borrow::Cow<'a, str>),
	Number(u64)
}

fn tokenize(s: &[u8]) -> Vec<Token> {
	let mut out = vec![];
	let mut ptr = 0;

	while ptr < s.len() {
		match s[ptr] {
			b';' => { // Comment. Consume rest of line.
				ptr += 1;
				while ptr < s.len() && s[ptr] != b'\n' {
					ptr += 1;
				}
			},

			b'a'..=b'z' | b'A'..=b'Z' => {
				let start = ptr;

				ptr += 1;
				while ptr < s.len() && matches!(s[ptr], b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9') {
					ptr += 1;
				}

				out.push(Token::Ident(&s[start..ptr]));
			},
			
			b'0'..=b'9' => {
				let start = ptr;

				ptr += 1;
				while ptr < s.len() && matches!(s[ptr], b'0'..=b'9') {
					ptr += 1;
				}

				out.push(Token::Number(atoi::atoi(&s[start..ptr]).unwrap()));
			},

			b'"' => {
				let start = ptr + 1;

				ptr += 1;
				loop {
					assert!(ptr < s.len(), "String without closing quote");

					if s[ptr] == b'"' {
						ptr += 1;
						break;
					}

					ptr += 1;
				}
				
				let slice = &s[start..ptr - 1];
				let str = core::str::from_utf8(slice)
					.expect("Invalid utf8");

				if str.contains("\\") {
					let escaped = str
						.replace("\\n", "\n")
						.replace("\\t", "\t")
						.replace("\\r", "\r");

					out.push(Token::String(std::borrow::Cow::Owned(escaped)));
				} else {
					out.push(Token::String(std::borrow::Cow::Borrowed(str)));
				}
			}
			
			b' ' | b'\t' | b'\n' | b'\r' => {
				ptr += 1;
			},
			
			b'(' => {
				ptr += 1;
				out.push(Token::LParen);
			},

			b')' => {
				ptr += 1;
				out.push(Token::RParen);
			}

			other => panic!("Unexpected: {}", other as char),
		}
	}

	out
}

#[derive(Debug, Clone)]
enum Node<'a> {
	Call(&'a [u8], Vec<Self>),
	Ident(&'a [u8]),
	String(std::borrow::Cow<'a, str>),
	Number(u64)
}

fn parse<'a>(toks: &'a [Token]) -> Vec<Node<'a>> {
	let mut out = vec![];
	let mut ind = 0;

	fn sexp<'a>(toks: &'a [Token], ind: &mut usize) -> Node<'a> {
		match &toks[*ind] {
			Token::LParen => {
				*ind += 1;

				match toks.get(*ind) {
					Some(Token::Ident(i)) => {
						*ind += 1;

						let mut args = vec![];
						while *ind < toks.len() && !matches!(toks[*ind], Token::RParen) {
							let e = sexp(toks, ind);
							args.push(e);
						}
						
						*ind += 1;
						return Node::Call(i, args);
					},
					Some(whatever) => panic!("Expected ident, not {whatever:#?}"),
					None => panic!("Expected name for function call")
				}
			},
			
			Token::Ident(i) => {
				*ind += 1;
				return Node::Ident(i);
			},
			
			Token::Number(n) => {
				*ind += 1;
				return Node::Number(*n);
			},
			
			Token::String(s) => {
				*ind += 1;
				return Node::String(s.to_owned());
			}

			whatever => panic!("Didn't expect {whatever:#?}")
		}
	}

	while ind < toks.len() {
		out.push(sexp(toks, &mut ind));
	}

	out
}

fn assemble<'a>(nodes: &'a [Node<'a>]) -> extern "C" fn() -> u64 {
	let mut out: Vec<u8> = vec![];
	let mut ind = 0;

	fn ident_to_register(i: &[u8]) -> u8 {
		match i {
			b"r0" => 0,
			b"rax" => 0,
			b"r1" => 1,
			b"rcx" => 1,
			b"r2" => 2,
			b"rdx" => 2,
			b"r3" => 3,
			b"rbx" => 3,
			b"r4" => 4,
			b"rsp" => 4,
			b"r5" => 5,
			b"rbp" => 5,
			b"r6" => 6,
			b"rsi" => 6,
			b"r7" => 7,
			b"rdi" => 7,
			whatever => panic!("Unknown register {}", core::str::from_utf8(whatever).unwrap())
		}
	}
	
	fn get_register<'a>(r: Option<&Node<'a>>) -> u8 {
		match r {
			Some(Node::Ident(i)) => ident_to_register(*i),
			whatever => panic!("Expected ident, got {whatever:#?}")
		}
	}
	
	enum Value {
		Register(u8),
		Imm(u64),
		SizedAddress(*const i8, usize),
		Stack,
		Void
	}

	fn get_value<'a>(v: Option<&Node<'a>>, out: &mut Vec<u8>) -> Value {
		assemble_exp(v.expect("Expected a value"), out)
	}

	fn assemble_exp<'a>(node: &'a Node<'a>, out: &mut Vec<u8>) -> Value {
		match node {
			Node::Ident(i) => Value::Register(ident_to_register(*i)),
			Node::Number(n) => Value::Imm(*n),
			Node::String(s) => Value::SizedAddress(s.as_ptr() as _, s.len()),
			Node::Call(name, args) => {
				match *name {
					b"set" => {
						let register = get_register(args.get(0));
						let value = get_value(args.get(1), out);

						match value {
							Value::Register(r) => out.extend(dasm::tier::raw::amd64::mov_r64_r64(register, r)),
							Value::Imm(i) => out.extend(dasm::tier::raw::amd64::mov_r64_i64(register, i)),
							Value::Stack => {
								out.extend(dasm::tier::raw::amd64::pop_r64(0));
								out.extend(dasm::tier::raw::amd64::mov_r64_r64(register, 0));
							},
							Value::Void => panic!("Expected value, got void"),
							Value::SizedAddress(addr, ..) => out.extend(dasm::tier::raw::amd64::mov_r64_i64(register, addr as _))
						}

						Value::Void
					},

					b"add" => {
						let lhs = get_value(args.get(0), out);
						let rhs = get_value(args.get(1), out);

						match lhs {
							Value::Register(r) => out.extend(dasm::tier::raw::amd64::push_r64(r)),
							Value::Imm(i) => out.extend(dasm::tier::raw::amd64::push_i32(i as u32)),
							Value::Stack => (),
							Value::Void => panic!("Expected value, got void"),
							Value::SizedAddress(..) => todo!("address")
						};
						
						match rhs {
							Value::Register(r) => out.extend(dasm::tier::raw::amd64::push_r64(r)),
							Value::Imm(i) => out.extend(dasm::tier::raw::amd64::push_i32(i as u32)),
							Value::Stack => (),
							Value::Void => panic!("Expected value, got void"),
							Value::SizedAddress(..) => todo!("address")
						}

						out.extend(dasm::tier::raw::amd64::pop_r64(1)); // rcx = pop()
						out.extend(dasm::tier::raw::amd64::pop_r64(0)); // rax = pop()
						out.extend(dasm::tier::raw::amd64::add_r64_r64(0, 1)); // rax += rcx
						out.extend(dasm::tier::raw::amd64::push_r64(0));

						Value::Stack
					},

					b"syscall" => {
						out.extend(dasm::tier::raw::amd64::syscall());
						Value::Void
					},

					b"ret" => {
						let val = get_value(args.get(0), out);

						match val {
							Value::Imm(i) => out.extend(dasm::tier::raw::amd64::mov_r64_i64(0, i)),
							Value::Register(r) => out.extend(dasm::tier::raw::amd64::mov_r64_r64(0, r)),
							Value::Stack => out.extend(dasm::tier::raw::amd64::pop_r64(0)),
							Value::SizedAddress(addr, ..) => out.extend(dasm::tier::raw::amd64::mov_r64_i64(0, addr as _)),
							Value::Void => ()
						}

						out.extend(dasm::tier::raw::amd64::ret());
						Value::Void
					},
					
					b"print" => {
						let val = get_value(args.get(0), out);
						match val {
							Value::SizedAddress(addr, len) => {
								out.extend(dasm::tier::raw::amd64::mov_r64_i64(6, addr as _));
								out.extend(dasm::tier::raw::amd64::mov_r64_i64(2, len as _));
								out.extend(dasm::tier::raw::amd64::mov_r64_i64(0, 1));
								out.extend(dasm::tier::raw::amd64::mov_r64_i64(7, 1));
								out.extend(dasm::tier::raw::amd64::syscall());
								
								// Print the newline
								out.extend(dasm::tier::raw::amd64::mov_r64_i64(6, b"\n".as_ptr() as _));
								out.extend(dasm::tier::raw::amd64::mov_r64_i64(2, 1));
								out.extend(dasm::tier::raw::amd64::mov_r64_i64(0, 1));
								out.extend(dasm::tier::raw::amd64::mov_r64_i64(7, 1));
								out.extend(dasm::tier::raw::amd64::syscall());

								Value::Void
							},
							_ => todo!("everything else")
						}
					}

					whatever => todo!("Not sure what {} is", core::str::from_utf8(whatever).unwrap())
				}
			},
		}
	}

	while ind < nodes.len() {
		match assemble_exp(&nodes[ind], &mut out) {
			Value::Stack => out.extend(dasm::tier::raw::amd64::pop_r64(0)),
			_ => ()
		}

		ind += 1;
	}

	out.extend(dasm::tier::raw::amd64::ret());

	let mapped = dasm::mmap::Mmap::exec(&out)
		.expect("Failed to mmap");

	let f = unsafe { std::mem::transmute(mapped.as_ptr()) };

	std::mem::forget(mapped);

	f
}

fn main() {
	let tokens = tokenize(br#"
		; Print out a string manually
		(set rsi "Hello, world!\n") ; address
		(set rdx 14) ; length
		(set rdi 1) ; fd (stdout)
		(set rax 1) ; sys_write
		(syscall)

		; Identical behavior to this
		(print "Hello, world!")

		; Not actually needed since a return is automatically inserted for you
		; Nest however many you want. Uses the stack.
		(ret (add 2 (add 2 4)))
	"#);

	let nodes = parse(&tokens);

	let out = assemble(&nodes);
	assert_eq!(out(), 2 + 2 + 4);
}
