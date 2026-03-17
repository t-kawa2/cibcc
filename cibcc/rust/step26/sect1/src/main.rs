mod tokenize;
mod parser;
mod codegen;
mod r_type;

use std::env;
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
	let input = &args[1];
	let tokens = tokenize(input);
	
	let mut parser = Parser::new(tokens);
	let mut prog = parser.program();
	add_type(&mut prog);

	let assembly = codegen(prog);

	println!("{}", assembly);
}

