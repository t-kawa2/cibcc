use crate::tokenize::Token;
use crate::tokenize::TokenKind;

#[derive(Debug, PartialEq, Clone)]
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
	pub name: String,
	pub offset: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarList {
	pub next: Option<Box<VarList>>,
	pub var: Option<Box<Var>>,
}

#[derive(Debug, PartialEq)]
pub struct Function {
	pub next: Option<Box<Function>>,
	pub name: String,
	pub params: Option<Box<VarList>>,
	pub node: Option<Box<Node>>,
	pub locals: Option<Box<VarList>>,
	pub stack_size: i64,
}

impl Function {
	fn new(name: String, params: Option<Box<VarList>>, node: Option<Box<Node>>, locals: Option<Box<VarList>>) -> Self {
		Self {
			next: None,
			name,
			params,
			node,
			locals,
			stack_size: 0,
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
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
			funcname: String::new(),
			args: None,
			var: None,
		}
	}
	fn new_binop(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
		Box::new(Node {
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
		Box::new(Node {
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
		Box::new(Node {
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
		Box::new(Node {
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
	pub fn new_block() -> Self {
		Self::new(NodeKind::Block)
	}
}

pub struct Parser {
	pub tokens: Vec<Token>,
	pub pos: usize,
	pub locals: Option<Box<VarList>>,
	pub current_fn_name: String,
	pub stack_size: i64,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			tokens,
 			pos: 0,
			locals: None,
			current_fn_name: String::new(),
			stack_size: 0,
		}
	}
	fn peek(&self) -> &Token {
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
		while let Some(vl) = cur {
			if let Some(ref v) = vl.var {
				if v.name == name {
					return Some((**v).clone());
				}
			}
			cur = &vl.next;
		}
		None
	}
	fn push_var(&mut self, name: String) -> Var {
		self.stack_size += 8;

		let var = Var {
			name: name.clone(),
			offset: self.stack_size,
		};
		let new_vl = Box::new(VarList {
			next: self.locals.take(),
			var: Some(Box::new(var.clone())),
		});
		self.locals = Some(new_vl);

		var
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
	fn expect_ident(&mut self) -> String {
		let name = if let TokenKind::Ident(name) = &self.peek().kind {
			name.clone()
		} else {
			panic!("数値を期待しましたが、違いました: {:?}", self.peek());
		};

		self.pos += 1;
		name
	}
	fn read_func_params(&mut self) -> Option<Box<VarList>> {
		if self.consume(")") {
			return None;
		}
		let name = self.expect_ident();
		let var_body = self.push_var(name);

		let mut head = Box::new(VarList {
			next: None,
			var: Some(Box::new(var_body)),
		});

		let mut cur = &mut head;

		while !self.consume(")") {
			self.expect(",");

			let name = self.expect_ident();
			let var_body = self.push_var(name);

			let next_node = Box::new(VarList {
				next: None,
				var: Some(Box::new(var_body)),
			});

			cur.next = Some(next_node);
			cur = cur.next.as_mut().unwrap();
		}
		Some(head)
	}
	pub fn function(&mut self) -> Function {
		let name = self.expect_ident();
		self.locals = None;
		self.stack_size = 0;
		self.current_fn_name = name.clone();
		self.expect("(");
		let params = self.read_func_params();
		self.expect("{");

		let mut head = Box::new(Node::new_block());
		let mut cur = &mut *head;

		while !self.consume("}") {
			let stmt = self.stmt();
			cur.next = Some(Box::new(*stmt));
			cur = cur.next.as_mut().unwrap();
		}

		Function::new(name, params, Some(Box::new(*head)), self.locals.take())
	}
	pub fn program(&mut self) -> Function {
		let mut head = self.function();
		let mut cur = &mut head;

		while !self.is_eof() {
			let next_func = self.function();
			cur.next = Some(Box::new(next_func));
			cur = cur.next.as_mut().unwrap();
		}
		head
	}

	fn stmt(&mut self) -> Box<Node> {
		if self.consume("return") {
			let mut node = Node::new_unary(NodeKind::Return, self.expr());
			node.funcname = self.current_fn_name.clone();
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
				node.els= Some(self.stmt());
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
			node = Node::new_binop(NodeKind::Assign, node, self.assign());
		}
		node
	}
	fn equality(&mut self) -> Box<Node> {
		let mut node = self.relational();

		loop {
			if self.consume("==") {
				node = Node::new_binop(NodeKind::EQ, node, self.relational());
			} else if self.consume("!=") {
				node = Node::new_binop(NodeKind::NE, node, self.relational());
			} else {
				return node;
			}
		}
	}
	fn relational(&mut self) -> Box<Node> {
		let mut node = self.add();

		loop {
			if self.consume("<") {
				node = Node::new_binop(NodeKind::LT, node, self.add());
			} else if self.consume("<=") {
				node = Node::new_binop(NodeKind::LE, node, self.add());
			} else if self.consume(">") {
				node = Node::new_binop(NodeKind::LT, self.add(), node);
			} else if self.consume(">=") {
				node = Node::new_binop(NodeKind::LE, self.add(), node);
			} else {
				return node;
			}
		}
	}
	
	fn add(&mut self) -> Box<Node> {
		let mut node = self.mul();

		loop {
			if self.consume("+") {
				node = Node::new_binop(NodeKind::Add, node, self.mul());
			} else if self.consume("-") {
				node = Node::new_binop(NodeKind::Sub, node, self.mul());
			} else {
				return node;
			}
		}
	}
	fn mul(&mut self) -> Box<Node> {
		let mut node = self.unary();

		loop {
			if self.consume("*") {
				node = Node::new_binop(NodeKind::Mul, node, self.unary());
			} else if self.consume("/") {
				node = Node::new_binop(NodeKind::Div, node, self.unary());
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
			return Node::new_binop(NodeKind::Sub, Node::new_num(0), self.unary());
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
					node.args = self.func_args();
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
