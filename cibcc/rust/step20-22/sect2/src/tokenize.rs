use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
	Reserved(String),
	Ident(String),
	Num(i64),
	Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
	pub kind: TokenKind,
	pub input: String,
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.input)
	}
}

pub fn tokenize(input: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut chars = input.chars().peekable();

	while let Some(c) = chars.next() {

		if c.is_whitespace() {
			continue;
		}
		match c {

			'a'..='z' | 'A'..='Z' | '_' => {
				let mut identifier = String::new();
				identifier.push(c);
				while let Some(&next_c) = chars.peek() {
					if next_c.is_alphanumeric() || next_c == '_' {
						identifier.push(chars.next().unwrap());
					} else {
						break;
					}
				}

				if identifier == "return" {
					tokens.push(Token{
						kind: TokenKind::Reserved(identifier.clone()),
						input: identifier,
					});
				} else if identifier == "if" {
					tokens.push(Token{
						kind: TokenKind::Reserved(identifier.clone()),
						input: identifier,
					});
				} else if identifier == "else" {
					tokens.push(Token{
						kind: TokenKind::Reserved(identifier.clone()),
						input: identifier,
					});
				} else if identifier == "while" {
					tokens.push(Token{
						kind: TokenKind::Reserved(identifier.clone()),
						input: identifier,
					});
				} else if identifier == "for" {
					tokens.push(Token{
						kind: TokenKind::Reserved(identifier.clone()),
						input: identifier,
					});
				} else if identifier == "int" {
					tokens.push(Token{
						kind: TokenKind::Reserved(identifier.clone()),
						input: identifier,
					});
				} else {
					tokens.push(Token{
						kind: TokenKind::Ident(identifier.clone()),
						input: identifier,
					});
				}
			}
			'0'..='9' => {
				let mut num_str = String::new();
				num_str.push(c);
				while let Some(&next_c) = chars.peek() {
					if next_c.is_ascii_digit() {
						num_str.push(chars.next().unwrap());
					} else {
						break;
					}
				}
				let val = num_str.parse::<i64>().unwrap();
				tokens.push(Token{
					kind: TokenKind::Num(val),
					input: num_str,
				});
			}
			'+' | '-' | '*' | '/' | '(' | ')' | ';' | '{' | '}' | ',' | '&' |
			'[' | ']' => {
				tokens.push(Token{
					kind: TokenKind::Reserved(c.to_string()),
					input: c.to_string(),
				});
			}
			'=' => {
				if chars.peek() == Some(&'=') {
					chars.next();
					tokens.push(Token{
						kind: TokenKind::Reserved("==".to_string()),
						input: "==".into(),
					});
				} else {
					tokens.push(Token{
						kind: TokenKind::Reserved("=".to_string()),
						input: "=".into(),
					});
				}
			}
			'!' => {
				if chars.peek() == Some(&'=') {
					chars.next();
					tokens.push(Token{
						kind: TokenKind::Reserved("!=".to_string()),
						input: "!=".into(),
					});
				} else {
					break;
				}
			}
			'<' => {
				if chars.peek() == Some(&'=') {
					chars.next();
					tokens.push(Token{
						kind: TokenKind::Reserved("<=".to_string()),
						input: "<=".into(),
					});
				} else {
					tokens.push(Token{
						kind: TokenKind::Reserved("<".to_string()),
						input: "<".into(),
					});
				}
			}
			'>' => {
				if chars.peek() == Some(&'=') {
					chars.next();
					tokens.push(Token{
						kind: TokenKind::Reserved(">=".to_string()),
						input: ">=".into(),
					});
				} else {
					tokens.push(Token{
						kind: TokenKind::Reserved(">".to_string()),
						input: ">".into(),
					});
				}
			}

			_ => panic!("予期しないもじです: {}", c),
		}
	}
	tokens.push(Token{
		kind: TokenKind::Eof,
		input: "".to_string(),
	});
	tokens
}

