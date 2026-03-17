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

	let mut parser = Parser::new(tokens);
	let mut prog = parser.program();

	let mut curr_fn = Some(&mut prog);
	while let Some(f) = curr_fn {
		let mut offset = 0;

		let mut v = f.locals.as_mut();
		while let Some(var_node) = v {
			offset += 8;
			var_node.offset = offset;
			v = var_node.next.as_mut();
		}
		f.stack_size = offset;
		curr_fn = f.next.as_deref_mut();
	}

	let assemble_code = codegen(prog);

	println!("{}", assemble_code);

}

