use std::env;
use std::process;

#[derive(Debug, PartialEq, Clone)]
enum TokenKind {
	Reserved(String),
	Num(i64),
	Eof,
}

#[derive(Debug, PartialEq, Clone)]
struct Token {
	kind: TokenKind,
	input: String,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
	let mut tokens: Vec<Token> = vec![];
	let mut chars = input.chars().peekable();

	while let Some(&c) = chars.peek() {
		match c {
			' ' | '\n' => {
				chars.next();
			}
			'+' | '-' | '*' | '/' | '(' | ')' => {
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
			_ => {
				return Err(format!("予期しない文字です: {}", c));
			}
		}
	}
	tokens.push(Token{
		kind: TokenKind::Eof,
		input: "".to_string(),
	});
	Ok(tokens)
}

#[derive(Debug, PartialEq)]
enum NodeKind {
	Add,
	Sub,
	Mul,
	Div,
	Num(i64),
}

#[derive(Debug, PartialEq)]
struct Node {
	kind: NodeKind,
	lhs: Option<Box<Node>>,
	rhs: Option<Box<Node>>,
}

impl Node {
	fn new_binary(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
		Box::new(Node{
			kind,
			lhs: Some(lhs),
			rhs: Some(rhs),
		})
	}
	fn new_num(val: i64) -> Box<Node> {
		Box::new(Node{
			kind: NodeKind::Num(val),
			lhs: None,
			rhs: None,
		})
	}
}

struct Parser {
	tokens: Vec<Token>,
	pos: usize,
}

impl Parser {
	fn new(tokens: Vec<Token>) -> Self {
		Self { tokens, pos: 0 }
	}
	fn peek(&self) ->&Token {
		&self.tokens[self.pos]
	}
	fn consume(&mut self, op: &str) -> bool {
		if let TokenKind::Reserved(ref s) = self.peek().kind {
			if s == op {
				self.pos += 1;
				return true;
			}
		}
		false
	}
	fn expect_number(&mut self) -> i64 {
		if let TokenKind::Num(val) = self.peek().kind {
			self.pos += 1;
			return val;
		}
		panic!("数値を期待しましたが、違いました: {:?}", self.peek());
	}
	fn expect(&mut self, op: &str) {
		if !self.consume(op) {
			panic!("'{}' を期待しましたが、違いました: {:?}", op, self.peek());
		}
	}
	fn expr(&mut self) -> Box<Node> {
		self.add()
	}
	fn add(&mut self) -> Box<Node> {
		let mut node = self.mul();

		loop {
			if self.consume("+") {
				node = Node::new_binary(NodeKind::Add, node, self.mul());
			} else if self.consume("-") {
				node = Node::new_binary(NodeKind::Sub, node, self.mul());
			} else {
				return node;
			}
		}
	}
	fn mul(&mut self) -> Box<Node> {
		let mut node = self.unary();

		loop {
			if self.consume("*") {
				node = Node::new_binary(NodeKind::Mul, node, self.unary());
			} else if self.consume("/") {
				node = Node::new_binary(NodeKind::Div, node, self.unary());
			} else {
				return node;
			}
		}
	}
	fn unary(&mut self) -> Box<Node> {
		if self.consume("+") {
			return self.unary();
		}
		if self.consume("-") {
			return Node::new_binary(NodeKind::Sub, Node::new_num(0), self.unary());
		}
		return self.primary();
	}
	fn primary(&mut self) -> Box<Node> {
		if self.consume("(") {
			let node = self.expr();
			self.expect(")");
			return node;
		}

		Node::new_num(self.expect_number())
	}
}

fn codegen(node: Box<Node>) -> String {
	let mut output = String::new();

	match node.kind {
		NodeKind::Num(val) => {
			output.push_str(&format!("  push {}\n", val));
		}
		NodeKind::Add | NodeKind::Sub | NodeKind::Mul | NodeKind::Div => {
			if let (Some(lhs), Some(rhs)) = (node.lhs, node.rhs) {
				output.push_str(&codegen(lhs));
				output.push_str(&codegen(rhs));

				output.push_str("  pop rdi\n");
				output.push_str("  pop rax\n");

				match node.kind {
					NodeKind::Add => output.push_str("  add rax, rdi\n"),
					NodeKind::Sub => output.push_str("  sub rax, rdi\n"),
					NodeKind::Mul => output.push_str("  imul rax, rdi\n"),
					NodeKind::Div => {
						output.push_str("  cqo\n");
						output.push_str("  idiv rdi\n");
					},
					_ => unreachable!(),
				}
				output.push_str("  push rax\n");
			}
		}
	}
	output
}

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

	let program = Parser::expr(&mut Parser::new(tokens));
	let assemble_code = codegen(program);

	println!(".intel_syntax noprefix");
	println!(".global main");
	println!("main:");
	println!("{}", assemble_code);

	println!("  pop rax");
	println!("  ret");
}

