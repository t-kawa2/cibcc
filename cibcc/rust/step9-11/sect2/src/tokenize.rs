#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
	Reserved(String),
	Num(i64),
	Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
	pub kind: TokenKind,
	pub input: String,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
	let mut tokens: Vec<Token> = vec![];
	let mut chars = input.chars().peekable();

	while let Some(&c) = chars.peek() {
		match c {
			' ' | '\n' => {
				chars.next();
			}
			'=' => {
				chars.next();
				if chars.peek() == Some(&'=') {
					chars.next();
					tokens.push(Token{
						kind: TokenKind::Reserved("==".to_string()),
						input: "==".into(),
					});
				} else {
					break;
				}
			}
			'!' => {
				chars.next();
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
				chars.next();
				if chars.peek() == Some(&'=') {
					chars.next();
					tokens.push(Token{
						kind: TokenKind::Reserved("<=".to_string()),
						input: "<=".into(),
					});
				} else {
					tokens.push(Token{
						kind: TokenKind::Reserved('<'.to_string()),
						input: "<".into(),
					});
				}
			}
			'>' => {
				chars.next();
				if chars.peek() == Some(&'=') {
					chars.next();
					tokens.push(Token{
						kind: TokenKind::Reserved(">=".to_string()),
						input: ">=".into(),
					});
				} else {
					tokens.push(Token{
						kind: TokenKind::Reserved('>'.to_string()),
						input: ">".into(),
					});
				}
			}
			'+' | '-' | '*' | '/' | '(' | ')' | ';' => {
				let s = c.to_string();
				tokens.push(Token{
					kind: TokenKind::Reserved(s.clone()),
					input: s,
				});
				chars.next();
			}
			'0'..='9' => {
				let mut num_str = String::new();
				while let Some(&d) = chars.peek() {
					if d.is_digit(10) {
						num_str.push(d);
						chars.next();
					} else {
						break;
					}
				}
				let value: i64 = num_str.parse().map_err(|_| format!("数値を解析できません: {}", num_str))?;
				tokens.push(Token{
					kind: TokenKind::Num(value),
					input: num_str,
				});
			}
			'a'..='z' => {
				let mut identifier = String::new();
				while let Some(&c) = chars.peek() {
					if c.is_alphanumeric() || c == '_' {
						identifier.push(c);
						chars.next();
					} else {
						break;
					}
				}
				if identifier == "return" {
					tokens.push(Token{
						kind: TokenKind::Reserved(identifier.clone()),
						input: identifier,
					});
				} else {
					return Err(format!("未知の識別子です: {}", identifier));
				}
			},
			_ => todo!()
		}
	}
	tokens.push(Token{
		kind: TokenKind::Eof,
		input: "".to_string(),
	});
	Ok(tokens)
}

