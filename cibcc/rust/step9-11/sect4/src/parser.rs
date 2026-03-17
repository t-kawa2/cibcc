use crate::tokenize::Token;
use crate::tokenize::TokenKind;

#[derive(Debug, PartialEq)]
pub enum NodeKind {
	Add,
	Sub,
	Mul,
	Div,
	EQ,
	NE,
	LT,
	LE,
	Assign,
	Return,
	ExprStmt,
	LVar,
	Num(i64),
}

#[derive(Debug, PartialEq)]
pub struct Node {
	pub kind: NodeKind,
	pub lhs: Option<Box<Node>>,
	pub rhs: Option<Box<Node>>,
	pub next: Option<Box<Node>>,
	pub name: String,
}

impl Node {
	fn new_binary(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
		Box::new(Node{
			kind,
			lhs: Some(lhs),
			rhs: Some(rhs),
			next: None,
			name: "".to_string(),
		})
	}
	fn new_unary(kind: NodeKind, lhs: Box<Node>) -> Box<Node> {
		Box::new(Node{
			kind,
			lhs: Some(lhs),
			rhs: None,
			next: None,
			name: "".to_string(),
		})
	}
	fn new_num(val: i64) -> Box<Node> {
		Box::new(Node{
			kind: NodeKind::Num(val),
			lhs: None,
			rhs: None,
			next: None,
			name: "".to_string(),
		})
	}
	fn new_var(name: String) -> Box<Node> {
		Box::new(Node{
			kind: NodeKind::LVar,
			lhs: None,
			rhs: None,
			next: None,
			name: name,
		})
	}
}

pub struct Parser {
	pub tokens: Vec<Token>,
	pub pos: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self { tokens, pos: 0 }
	}
	fn peek(&self) ->&Token {
		&self.tokens[self.pos]
	}
	fn next_token(&mut self) -> &Token {
		let tok = &self.tokens[self.pos];
		if self.pos < self.tokens.len() - 1 {
			self.pos += 1;
		}
		tok
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
	fn expect(&mut self, op: &str) {
		if !self.consume(op) {
			panic!("'{}' を期待しましたが、違いました: {:?}", op, self.peek());
		}
	}
	fn is_eof(&self) -> bool {
		matches!(self.peek().kind, TokenKind::Eof)
	}
	pub fn program(&mut self) -> Box<Node> {
		let mut head = self.stmt();
		let mut cur = &mut head;

		while !self.is_eof() {
			cur.next = Some(self.stmt());
			if let Some(ref mut next_node) = cur.next {
				cur = next_node;
			}
		}
		head
	}
	fn stmt(&mut self) -> Box<Node> {
		if self.consume("return") {
			let node = Node::new_unary(NodeKind::Return, self.expr());
			self.expect(";");
			return node;
		}

		let node = Node::new_unary(NodeKind::ExprStmt, self.expr());
		self.expect(";");
		node
	}
	fn expr(&mut self) -> Box<Node> {
		self.assign()
	}
	fn assign(&mut self) -> Box<Node> {
		let mut node = self.equality();

		if self.consume("=") {
			node = Node::new_binary(NodeKind::Assign, node, self.equality());
		}
		node
	}
	fn equality(&mut self) -> Box<Node> {
		let mut node = self.relation();

		loop {
			if self.consume("==") {
				node = Node::new_binary(NodeKind::EQ, node, self.relation());
			} else if self.consume("!=") {
				node = Node::new_binary(NodeKind::NE, node, self.relation());
			} else {
				return node;
			}
		}
	}
	fn relation(&mut self) -> Box<Node> {
		let mut node = self.add();

		loop {
			if self.consume("<") {
				node = Node::new_binary(NodeKind::LT, node, self.add());
			} else if self.consume("<=") {
				node = Node::new_binary(NodeKind::LE, node, self.add());
			} else if self.consume(">") {
				node = Node::new_binary(NodeKind::LT, self.add(), node);
			} else if self.consume(">=") {
				node = Node::new_binary(NodeKind::LE, self.add(), node);
			} else {
				return node;
			}
		}
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
		let tok = self.peek().clone();

		match tok.kind {
			TokenKind::Reserved(ref s) if s == "(" => {
				self.next_token();
				let node = self.expr();
				self.expect(")");
				node
			}
			TokenKind::Ident(name) => {
				self.next_token();
				Node::new_var(name)
			}
			TokenKind::Num(val) => {
				self.next_token();
				Node::new_num(val)
			}
			_ => panic!("予期しないトークンです: {:?}", tok),
		}
	}
}

