use std::env;

fn number(input: &mut String) -> String {
	let mut chars = "".to_string();
	loop {
		match input.chars().next() {
			Some(c) if c.is_ascii_digit() => {
				chars += &c.to_string();
				input.remove(0);
			}
			_ => {
				break;
			}
		}
	}
	return chars;
}

fn operator(input: &mut String) -> String {
	let mut op = "".to_string();
	match input.chars().next() {
		Some(c) if c == '+' || c == '-' => {
			op = c.to_string();
			input.remove(0);
		}
		_ => 'block: {
			break 'block;
		}
	}
	return op;
}

fn main() {
    let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		eprintln!("引数の個数が正しくありません");
		return;
	}

	println!(".intel_syntax noprefix");
	println!(".global main");
	println!("main:");

	let mut arg = args[1].clone();
	println!("  mov rax, {}", number(&mut arg));

	loop {
		if arg.len() == 0 {
			break;
		}
		let op = operator(&mut arg);
		match op.as_str() {
			"+" => println!("  add rax, {}", number(&mut arg)),
			"-" => println!("  sub rax, {}", number(&mut arg)),
			_ => break,
		}
	}
	println!("  ret");
}

