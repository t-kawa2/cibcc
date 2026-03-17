use std::env;

#[derive(Debug, Clone)]
struct Token {
	number: Option<i64>,
	operator: Option<String>,
}

impl Token {
	fn number(num: i64) -> Self {
		Token {
			number: Some(num),
			operator: None,
		}
	}
	fn operator(op: String) -> Self {
		Token {
			number: None,
			operator: Some(op),
		}
	}
	fn parse(input: String) -> Vec<Token> {
		let mut tokens: Vec<Token> = vec![];
		let mut input = input;

		loop {
			if input.is_empty() {
				break;
			}
			consume_whitespace(&mut input);
			if let Some(token) = consume_number(&mut input) {
				tokens.push(token);
				continue;
			}
			if let Some(token) = consume_operator(&mut input) {
				tokens.push(token);
				continue;
			}
		}
		return tokens
	}
}


fn consume_whitespace(input: &mut String) {
	loop {
		match input.chars().next() {
			Some(c) if c.is_whitespace() => {
				input.remove(0);
			}
			_ => {
				break;
			}
		}
	}
}

fn consume_number(input: &mut String) -> Option<Token> {
	let mut digits = "".to_string();
	loop {
		match input.chars().next() {
			Some(c) if c.is_ascii_digit() => {
				digits += &c.to_string();
				input.remove(0);
			}
			_ => {
				break;
			}
		}
	}
	if digits.is_empty() {
		None
	} else {
		Some(Token::number(digits.parse::<i64>().unwrap()))
	}
}

fn consume_operator(input: &mut String) -> Option<Token> {
	match input.chars().next() {
		Some(c) if c == '+' || c == '-' => {
			input.remove(0);
			Some(Token::operator(c.to_string()))
		}
		_ => None
	}
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

	let arg: &String = &args[1];
	let mut tokens = Token::parse(arg.to_string());

	let num1 = tokens[0].number.unwrap();
	tokens.remove(0);
	println!("  mov rax, {}", num1);

	loop {
		if tokens.len() == 0 {
			break;
		}
		let op = tokens[0].operator.clone().unwrap();
		tokens.remove(0);
		let num2 = tokens[0].number.clone().unwrap();
		tokens.remove(0);

		if op == "+" {
			println!("  add rax, {}", num2);
		}
		if op == "-" {
			println!("  sub rax, {}", num2);
		}
	}
	println!("  ret");
}

