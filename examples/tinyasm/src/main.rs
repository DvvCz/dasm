#[derive(Debug, Clone)]
enum Token<'a> {
	LParen,
	RParen,
	Ident(&'a [u8]),
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
			
			_ => {
				panic!("??")
			}
		}
	}

	out
}

#[derive(Debug, Clone)]
enum Node<'a> {
	Call(&'a [u8], Vec<Self>),
	Ident(&'a [u8]),
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

			whatever => panic!("Didn't expect {whatever:#?}")
		}
	}

	while ind < toks.len() {
		out.push(sexp(toks, &mut ind));
	}

	out
}

fn assemble<'a>(nodes: Vec<Node<'a>>) -> extern "C" fn() -> u64 {
	let mut out: Vec<u8> = vec![];
	let mut ind = 0;
	
	fn get_register<'a>(r: Option<&Node<'a>>) -> u8 {
		match r {
			Some(Node::Ident(i)) => {
				match *i {
					b"r0" => 0,
					b"rax" => 0,
					b"r1" => 1,
					b"rcx" => 1,
					b"r2" => 2,
					b"rdx" => 2,
					b"r3" => 3,
					b"rbx" => 3,
					b"r4" => 4,
					b"rsi" => 4,
					b"r5" => 5,
					b"rdi" => 5,
					b"r6" => 6,
					b"rsp" => 6,
					b"r7" => 7,
					b"rbp" => 7,
					whatever => panic!("Unknown register {}", core::str::from_utf8(whatever).unwrap())
				}
			},
			whatever => panic!("Expected ident, got {whatever:#?}")
		}
	}
	
	fn get_value<'a>(v: Option<&Node<'a>>) -> u64 {
		match v {
			Some(Node::Ident(_)) => todo!("variable set"),
			Some(Node::Number(n)) => *n,
			Some(_) => todo!("nested set"),
			None => panic!("expected value to set to")
		}
	}

	while ind < nodes.len() {
		match &nodes[ind] {
			Node::Call(name, args) => {
				ind += 1;

				match *name {
					b"set" => {
						assert!(args.len() == 2, "Should only pass two arguments to set");

						let register = get_register(args.get(0));
						let value = get_value(args.get(1));

						out.extend(dasm::tier::raw::amd64::mov_r64_i64(register, value));
					},

					b"add" => {
						assert!(args.len() == 2, "Should only pass two arguments to set");

						let dst = get_register(args.get(0));
						let src = get_register(args.get(1));

						out.extend(dasm::tier::raw::amd64::add_r64_r64(dst, src));
					}
					
					b"ret" => {
						out.extend(dasm::tier::raw::amd64::ret());
					}

					whatever => todo!("Not sure what {} is", core::str::from_utf8(whatever).unwrap())
				}
			},
			
			whatever => todo!("Whatever this is {whatever:#?}")
		}
	}

	out.extend(dasm::tier::raw::amd64::ret());

	let mapped = dasm::mmap::Mmap::exec(&out)
		.expect("Failed to mmap");

	let f = unsafe { std::mem::transmute(mapped.as_ptr()) };

	std::mem::forget(mapped);

	f
}

fn main() {
	let tokens = tokenize(b"
		(set rax 11)
		(set rbx 22)
		(add rax rbx)
		(ret)
	");

	let nodes = parse(&tokens);
	
	let out = assemble(nodes);
	println!("{}", out());
}
