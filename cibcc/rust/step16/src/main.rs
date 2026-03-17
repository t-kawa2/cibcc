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
	let tokens = tokenize(input);
	
	let mut parser = Parser::new(tokens);
	let mut prog = parser.program();

	let mut curr_fn = Some(&mut prog);
	while let Some(f) = curr_fn {
		let mut offset = 0;

		let mut v = f.locals.as_mut();
		while let Some(var_node) = v {
			offset += 8;
			if let Some(ref mut var_box)= var_node.var {
				var_box.offset = offset;
			}
			v = var_node.next.as_mut();
		}

		f.stack_size = offset;
		curr_fn = f.next.as_deref_mut();
	}

	let assembly = codegen(prog);

	println!("{}", assembly);
}

