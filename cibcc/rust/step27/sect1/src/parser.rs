use crate::tokenize::Token;
use crate::tokenize::TokenKind;
use crate::r_type::Type;
use crate::r_type::array_of;

#[derive(Debug, Clone, PartialEq)]
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
	Addr,
	Deref,
	Return,
	If,
	While,
	For,
	Block,
	Funcall,
	ExprStmt,
	StmtExpr,
	Var,
	Num(i64),
	Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
	pub name: String,
	pub ty: Type,
	pub is_local: bool,
	pub offset: i64,
	pub contents: String,
	pub cont_len: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarList {
	pub next: Option<Box<VarList>>,
	pub var: Var,
}

impl VarList {
	pub fn new(var: Var) -> Self {
		Self {
			next: None,
			var,
		}
	}
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

#[derive(Debug, PartialEq)]
pub struct Program {
	pub globals: Option<VarList>,
	pub fns: Option<Function>,
}

#[derive(Debug, Clone, PartialEq)]
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
	pub ty: Option<Type>,
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
			ty: None,
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
			ty: None,
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
			ty: None,
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
			ty: None,
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
			ty: None,
		})
	}
}

pub struct Parser {
	pub tokens: Vec<Token>,
	pub pos: usize,
	pub locals: Option<Box<VarList>>,
	pub globals: Option<Box<VarList>>,
	pub current_fn_name: String,
	pub stack_size: i64,
	pub label_count: i32,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			tokens,
			pos: 0,
			locals: None,
			globals: None,
			current_fn_name: String::new(),
			stack_size: 0,
			label_count: 0,
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
	fn is_eof(&self) -> bool {
		matches!(self.peek().kind, TokenKind::Eof)
	}
	fn find_var(&self, name: &str) -> Option<Var> {
		let mut cur = &self.locals;
		while let Some(vl) = cur {
			if vl.var.name == name {
				return Some(vl.var.clone());
			}
			cur = &vl.next;
		}
		let mut cur = &self.globals;
		while let Some(vl) = cur {
			if vl.var.name == name {
				return Some(vl.var.clone());
			}
			cur = &vl.next;
		}
		None
	}
	fn push_var(&mut self, name: String, ty: Type, is_local: bool, contents: String, cont_len: usize) -> Var {
		if is_local {
			self.stack_size += ty.size() as i64;
		}

		let var = Var{
			name: name.clone(),
			ty: ty.clone(),
			is_local,
			offset: 0,
			contents,
			cont_len: cont_len as i64,
		};

		let next_list = if is_local { self.locals.take() } else { self.globals.take() };

		let new_vl = Box::new(VarList{
			next: next_list,
			var: var.clone(),
		});

		if is_local {
			self.locals = Some(new_vl);
		} else {
			self.globals = Some(new_vl);
		}
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
	fn check_ident(&self) -> bool {
		let tok = &self.tokens[self.pos];
		match tok.kind {
			TokenKind::Ident(_) => true,
			_ => false,
		}
	}
	fn read_func_params(&mut self) -> Option<Box<VarList>> {
		if self.consume(")") {
			return None;
		}

		let mut head = self.read_func_param();
		let mut cur = &mut head;

		while !self.consume(")") {
			self.expect(",");
			if let Some(ref mut vl) = cur {
				vl.next = self.read_func_param();
				cur = &mut vl.next;
			}
		}
		head
	}
	fn read_func_param(&mut self) -> Option<Box<VarList>> {
		let mut ty = self.basetype();
		let name = self.expect_ident();
		ty = self.read_type_suffix(ty);

		Some(Box::new(VarList::new(self.push_var(name, ty, true, "".to_string(), 0))))
	}
	fn declaration(&mut self) -> Node {
		let mut ty = self.basetype();
		let name = self.expect_ident();
		ty = self.read_type_suffix(ty);
		let var = self.push_var(name, ty, true, "".to_string(), 0);

		if self.consume(";") {
			return Node::new(NodeKind::Null);
		}

		self.expect("=");
		let lhs = Node::new_var(var);
		let rhs = self.expr();
		self.expect(";");
		let node = Node::new_binary(NodeKind::Assign, lhs, rhs);
		return *Node::new_unary(NodeKind::ExprStmt, node);
	}
	fn basetype(&mut self) -> Type {
		let mut ty = if self.consume("char") {
			Type::char_type()
		} else {
			self.expect("int");
			Type::int_type()
		};
		while self.consume("*") {
			ty = Type::pointer_to(ty);
		}
		ty
	}
	fn is_typename(&mut self) -> bool {
		if let TokenKind::Reserved(ref s) = self.peek().kind {
			return s == "int" || s == "char";
		}
		false
	}
	fn read_type_suffix(&mut self, mut base: Type) -> Type {
		if !self.consume("[") {
			return base;
		}
		let sz = self.expect_number();
		self.expect("]");
		base = self.read_type_suffix(base);
		return array_of(base, sz);
	}

	fn assign_type(&self, mut node: Box<Node>) -> Box<Node> {
		crate::r_type::visit(&mut node);

		node
	}
	fn is_function(&mut self) -> bool {
		let start_tok = self.pos;
		self.basetype();

		let isfunc = self.check_ident() && {
			self.pos += 1;
			self.consume("(")
		};

		self.pos = start_tok;
		isfunc
	}
	fn global_var(&mut self) {
		let mut ty = self.basetype();
		let name = self.expect_ident();
		ty = self.read_type_suffix(ty);
		self.expect(";");
		self.push_var(name, ty, false, "".to_string(), 0);
	}
	fn new_label(&mut self) -> String {
		let label = format!(".L.data.{}", self.label_count);
		self.label_count += 1;
		label
	}
	fn stmt_expr(&mut self) -> Box<Node> {
		let mut node = Node::new(NodeKind::StmtExpr);
		let mut head = Node::new(NodeKind::Null);
		let mut cur = &mut head;

		while !self.consume("}") {
			cur.next = Some(self.stmt());
			cur = cur.next.as_mut().unwrap();
		}
		self.expect(")");
		node.body = head.next;
		node.ty = Some(Type::int_type());

		Box::new(node)
}





	pub fn function(&mut self) -> Function {
		self.locals = None;
		self.stack_size = 0;

		self.basetype();
		let name = self.expect_ident();
		self.expect("(");
		let mut params = self.read_func_params();
		self.expect("{");

		let mut head = Node::new(NodeKind::Null);
		let mut cur = &mut head;

		while !self.consume("}") {
			let stmt = if self.is_typename() {
				self.declaration()
			} else {
				*self.stmt()
			};
			cur.next = Some(Box::new(stmt));
			cur = cur.next.as_mut().unwrap();
		}

		let mut offset = 0;
		let mut vl_ptr = self.locals.as_deref_mut();
		while let Some(vl) = vl_ptr {
			offset += vl.var.ty.size() as i64;
			vl.var.offset = offset;
			vl_ptr = vl.next.as_deref_mut();
		}

		let mut p_ptr = params.as_deref_mut();
		while let Some(p_vl) = p_ptr {
			let mut l_ptr = self.locals.as_deref();
			while let Some(l_vl) = l_ptr {
				if p_vl.var.name == l_vl.var.name {
					p_vl.var.offset = l_vl.var.offset;
					break;
				}
				l_ptr = l_vl.next.as_deref();
			}
			p_ptr = p_vl.next.as_deref_mut();
		}
		let mut body_node = Node::new(NodeKind::Block);
		body_node.body = head.next.take();
		self.stack_size = (offset + 7) / 8 * 8;

		let mut f = Function::new(name, params, Some(Box::new(body_node)), self.locals.take());
		f.stack_size = self.stack_size;

		f
	}
	pub fn program(&mut self) -> Program {
		let mut fns_head: Option<Box<Function>> = None;
		let mut fns_cur: *mut Option<Box<Function>> = &mut fns_head;

		while !self.is_eof() {
			if self.is_function() {
				let func = self.function();
				unsafe {
					*fns_cur = Some(Box::new(func));
					if let Some(ref mut node) = *fns_cur {
						fns_cur = &mut node.next;
					}
				}
			} else {
				self.global_var();
			}
		}
		Program {
			globals: self.globals.take().map(|b| *b),
			fns: fns_head.map(|b| *b),
		}
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
			let mut head = Node::new(NodeKind::Null);
			let mut cur = &mut head;

			while !self.consume("}") {
				let stmt = self.stmt();
				cur.next = Some(stmt);
				cur = cur.next.as_mut().unwrap();
			}

			node.body = head.next.take();
			return Box::new(node);
		}

		if self.is_typename() {
			return Box::new(self.declaration());
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
			node = Node::new_binary(NodeKind::Assign, node, self.assign());
		}
		node
	}
	fn equality(&mut self) -> Box<Node> {
		let mut node = self.relational();

		loop {
			if self.consume("==") {
				node = Node::new_binary(NodeKind::EQ, node, self.relational());
			} else if self.consume("!=") {
				node = Node::new_binary(NodeKind::NE, node, self.relational());
			} else {
				return node;
			}
		}
	}
	fn relational(&mut self) -> Box<Node> {
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
				let lhs = node;
				let rhs = self.mul();
				let mut new_node = Node::new_binary(NodeKind::Add, lhs, rhs);

				if let Some(ref lty) = new_node.lhs.as_ref().unwrap().ty {
					new_node.ty = Some(lty.clone());
				} else if let Some(ref rty) = new_node.rhs.as_ref().unwrap().ty {
					new_node.ty = Some(rty.clone());
				}
				node = new_node;
			} else if self.consume("-") {
				let lhs = node;
				let rhs = self.mul();
				let mut new_node = Node::new_binary(NodeKind::Sub, lhs, rhs);

				if let Some(ref lty) = new_node.lhs.as_ref().unwrap().ty {
					new_node.ty = Some(lty.clone());
				}
				node = new_node;
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
		if self.consume("&") {
			let lhs = self.unary();
			let mut node =  Node::new_unary(NodeKind::Addr, lhs);

			if let Some(ref lhs_ty) = node.lhs.as_ref().unwrap().ty {
				node.ty = Some(Type::pointer_to(lhs_ty.clone()));
			}
			return node;
		}
		if self.consume("*") {
			let lhs = self.unary();
			let mut node = Node::new_unary(NodeKind::Deref, lhs);

			if let Some(ref lhs_ty) = node.lhs.as_ref().unwrap().ty {
				node.ty = lhs_ty.base.as_ref().map(|b| (**b).clone());
			}
			return node;
		}
		if self.consume("sizeof") {
			let node = self.unary();
			let node = self.assign_type(node);
			let size = node.ty.as_ref().map_or(0, |ty| ty.size());
			return Node::new_num(size.try_into().unwrap());
		}
		return self.post_fix();
	}
	fn post_fix(&mut self) -> Box<Node> {
		let mut node = self.primary();

		while self.consume("[") {
			let exp = Node::new_binary(NodeKind::Add, node, self.expr());
			self.expect("]");
			let deref_node = Node::new_unary(NodeKind::Deref, exp);
			node = self.assign_type(deref_node);
		}
		return node;
	}
	fn primary(&mut self) -> Box<Node> {
		let tok = self.peek().clone();

		match tok.kind {
			TokenKind::Reserved(ref s) if s == "(" => {
				self.next_token();

				if self.consume("{") {
					let node = self.stmt_expr();
					return node;
				}

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
					Some(v) => v.clone(),
					None => panic!("未定義の変数です: {}", name),
				};
				let mut node_inner = Node::new_var(var.clone());
				node_inner.ty = Some(var.ty.clone());
				return node_inner;
			}
			TokenKind::Str => {
				let (contents, cont_len) = {
					let tok = self.next_token();
					(tok.contents.clone(), tok.cont_len)
				};

				let ty = array_of(Type::char_type(), cont_len);
				let label = self.new_label();

				let var = self.push_var(label.clone(), ty, false, contents, cont_len as usize);
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

