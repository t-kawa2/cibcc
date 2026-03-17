mod tokenize;
mod parser;
mod codegen;

use std::env;
use std::process;
use parser::Parser;
use tokenize::tokenize;
use codegen::codegen;

fn main() {
    let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		eprintln!("引数の個数が正しくありません");
		process::exit(1);
	}

	let input = &args[1];
	let tokens = match tokenize(input) {
		Ok(t) => t,
		Err(e) => {
			eprintln!("字句解析エラー: {}", e);
			process::exit(1);
		}
	};

	let program = Parser::stmt(&mut Parser::new(tokens));
	let assemble_code = codegen(program);

	println!(".intel_syntax noprefix");
	println!(".global main");
	println!("main:");
	println!("{}", assemble_code);

	println!("  pop rax");
	println!("  ret");
}

