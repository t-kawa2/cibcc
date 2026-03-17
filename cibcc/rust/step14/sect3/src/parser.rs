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
	If,
	While,
	For,
	Block,
	Funcall,
	ExprStmt,
	Var,
	Num(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
	pub next: Option<Box<Var>>,
	pub name: String,
	pub offset: i64,
}

#[derive(Debug, PartialEq)]
pub struct Program {
	pub node: Option<Box<Node>>,
	pub locals: Option<Box<Var>>,
	pub stack_size: i64,
}

#[derive(Debug, PartialEq)]
pub struct Node {
	pub kind: NodeKind,
	pub lhs: Option<Box<Node>>,
	pub rhs: Option<Box<Node>>,
	pub next: Option<Box<Node>>,
	pub cond: Option<Box<Node>>,
	pub then: Option<Box<Node>>,
	pub els: Option<Box<Node>>,
	pub init: Option<Box<Node>>,
	pub inc: Option<Box<Node>>,
	pub body: Option<Box<Node>>,
	pub funcname: String,
	pub args: Option<Box<Node>>,
	pub var: Option<Var>,
}

impl Node {
	fn new(kind: NodeKind) -> Self {
		Self {
			kind,
			lhs: None,
			rhs: None,
			next: None,
			cond: None,
			then: None,
			els: None,
			init: None,
			inc: None,
			body: None,
			funcname: "".to_string(),
			args: None,
			var: None,
		}
	}
	fn new_binary(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
		Box::new(Node{
			kind,
			lhs: Some(lhs),
			rhs: Some(rhs),
			next: None,
			cond: None,
			then: None,
			els: None,
			init: None,
			inc: None,
			body: None,
			funcname: "".to_string(),
			args: None,
			var: None,
		})
	}
	fn new_unary(kind: NodeKind, lhs: Box<Node>) -> Box<Node> {
		Box::new(Node{
			kind,
			lhs: Some(lhs),
			rhs: None,
			next: None,
			cond: None,
			then: None,
			els: None,
			init: None,
			inc: None,
			body: None,
			funcname: "".to_string(),
			args: None,
			var: None,
		})
	}
	fn new_num(val: i64) -> Box<Node> {
		Box::new(Node{
			kind: NodeKind::Num(val),
			lhs: None,
			rhs: None,
			next: None,
			cond: None,
			then: None,
			els: None,
			init: None,
			inc: None,
			body: None,
			funcname: "".to_string(),
			args: None,
			var: None,
		})
	}
	fn new_var(var: Var) -> Box<Node> {
		Box::new(Node{
			kind: NodeKind::Var,
			lhs: None,
			rhs: None,
			next: None,
			cond: None,
			then: None,
			els: None,
			init: None,
			inc: None,
			body: None,
			funcname: "".to_string(),
			args: None,
			var: Some(var),
		})
	}
}

pub struct Parser {
	tokens: Vec<Token>,
	pos: usize,
	locals: Option<Box<Var>>,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			tokens,
			pos: 0,
			locals: None,
		}
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
	fn find_var(&self, name: &str) -> Option<Var> {
		let mut cur = &self.locals;
		while let Some(ref var) = cur {
			if var.name == name {
				return Some((**var).clone());
			}
			cur = &var.next;
		}
		None
	}
	fn push_var(&mut self, name: String) -> Var {
		let last_offset = match &self.locals {
			Some(var) => var.offset,
			None => 0,
		};

		let var = Var{
			next: self.locals.take(),
			name: name,
			offset: last_offset + 8,
		};
		let new_var = Box::new(var);
		self.locals = Some(new_var.clone());
		*new_var
	}
	fn read_expr_stmt(&mut self) -> Box<Node> {
		return Node::new_unary(NodeKind::ExprStmt, self.expr());
	}
	fn func_args(&mut self) -> Option<Box<Node>> {
		if self.consume(")") {
			return None;
		}

		let mut head = self.assign();
		let mut cur = &mut head;
		while self.consume(",") {
			let next_node = self.assign();
			cur.next = Some(next_node);
			cur = cur.next.as_mut().unwrap();
		}
		self.expect(")");
		Some(head)
	}


	pub fn program(&mut self) -> Program {
		let mut head = Node::new(NodeKind::ExprStmt);
		let mut cur = &mut head;

		while !self.is_eof() {
			let stmt = self.stmt();
			cur.next = Some(stmt);
			cur = cur.next.as_mut().unwrap();
		}

		let stack_size = match &self.locals {
			Some(var) => var.offset,
			None => 0,
		};

		Program{
			node: head.next,
			locals: self.locals.take(),
			stack_size: stack_size,
		}
	}
	fn stmt(&mut self) -> Box<Node> {
		if self.consume("return") {
			let node = Node::new_unary(NodeKind::Return, self.expr());
			self.expect(";");
			return node;
		} else if self.consume("if") {
			self.expect("(");
			let cond = self.expr();
			self.expect(")");
			let then = self.stmt();
			let mut node = Node::new(NodeKind::If);
			node.cond = Some(cond);
			node.then = Some(then);

			if self.consume("else") {
				node.els = Some(self.stmt());
			}
			return Box::new(node);
		} else if self.consume("while") {
			self.expect("(");
			let cond = self.expr();
			self.expect(")");
			let then = self.stmt();
			let mut node = Node::new(NodeKind::While);
			node.cond = Some(cond);
			node.then = Some(then);

			return Box::new(node);
		} else if self.consume("for") {
			let mut node = Node::new(NodeKind::For);
			self.expect("(");
			let init = if !self.consume(";") {
				let node = self.read_expr_stmt();
				self.expect(";");
				Some(node)
			} else {
				None
			};
			let cond = if !self.consume(";") {
				let node = self.expr();
				self.expect(";");
				Some(node)
			} else {
				None
			};
			let inc = if !self.consume(")") {
				let node = self.read_expr_stmt();
				self.expect(")");
				Some(node)
			} else {
				None
			};
			let then = self.stmt();
			node.init = init;
			node.cond = cond;
			node.inc = inc;
			node.then = Some(then);

			return Box::new(node);
		} else if self.consume("{") {
			let mut node = Node::new(NodeKind::Block);
			let mut head = Node::new(NodeKind::Block);
			let mut cur = &mut head;

			while !self.consume("}") {
				let stmt = self.stmt();
				cur.next = Some(stmt);
				cur = cur.next.as_mut().unwrap();
			}

			node.body = head.next.take();
			return Box::new(node);
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
				if self.consume("(") {
					let mut node = Node::new(NodeKind::Funcall);
					node.funcname = name;
					node.body = self.func_args();
					return Box::new(node);
				}

				let var = match self.find_var(&name) {
					Some(v) => v,
					None => self.push_var(name),
				};
				Node::new_var(var)
			}
			TokenKind::Num(val) => {
				self.next_token();
				Node::new_num(val)
			}
			_ => panic!("予期しないトークンです: {:?}", tok),
		}
	}
}

