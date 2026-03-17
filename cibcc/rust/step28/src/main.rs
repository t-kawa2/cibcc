mod tokenize;
mod parser;
mod codegen;
mod r_type;

use std::env;
use std::fs;
use std::process;
use parser::Parser;
use tokenize::tokenize;
use codegen::codegen;
use crate::r_type::add_type;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		eprintln!("引数の個数が正しくありません");
		process::exit(1);
	}
	let filename = &args[1];
	let input = fs::read_to_string(filename).unwrap();
	let tokens = tokenize(&input);
	
	let mut parser = Parser::new(tokens);
	let mut prog = parser.program();

	let mut f = prog.fns.as_ref();
	while let Some(func) = f {
		f = func.next.as_deref();
	}
	add_type(&mut prog);

	let assembly = codegen(prog);

	println!("{}", assembly);
}

