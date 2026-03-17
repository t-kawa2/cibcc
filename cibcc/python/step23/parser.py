from r_type import *

class Node:
	def __init__(self, kind, lhs=None, rhs=None, cond=None, then=None, els=None, init=None, inc=None, body=None, funcname=None, args=None, val=None, next=None, var=None, ty=None):
		self.kind = kind
		self.lhs = lhs
		self.rhs = rhs
		self.cond = cond
		self.then = then
		self.els = els
		self.init = init
		self.inc = inc
		self.body = body
		self.funcname = funcname
		self.args = args
		self.val = val
		self.next = next
		self.var = var
		self.ty = ty

class Var:
	def __init__(self, name=None, ty=None, is_local=None, offset=None):
		self.name = name
		self.ty = ty
		self.is_local = is_local
		self.offset = offset

class VarList:
	def __init__(self, next=None, var=None):
		self.next = next
		self.var = var

class Function:
	def __init__(self, next=None, name=None, params=None, locals=None, stack_size=None):
		self.next = next
		self.name = name
		self.params = params
		self.locals = locals
		self.stack_size = stack_size

class Program:
	def __init__(self, globals=None, fns=None):
		self.globals = globals
		self.fns = fns

class Parser:
	def __init__(self, tokens):
		self.tokens = tokens
		self.pos = 0
		self.locals = None
		self.globals = None

	def consume(self, s):
		if self.tokens[self.pos].str != s:
			return False
		self.pos += 1
		return True

	def peek(self, s):
		if self.tokens[self.pos].kind != 'RESERVED' or self.tokens[self.pos].str != s:
			return None
		return self.tokens[self.pos]

	def current_peek(self):
		return self.tokens[self.pos]

	def expect(self, s):
		if not self.consume(s):
			raise Exception(f"expected {s}")

	def expect_number(self):
		if self.tokens[self.pos].kind != 'NUM':
			raise Exception("expected a number")
		val = self.tokens[self.pos].val
		self.pos += 1
		return val

	def new_num(self, val):
		node = Node('NUM')
		node.val = val
		return node

	def at_eof(self):
		return self.tokens[self.pos].kind == 'EOF'

	def consume_ident(self):
		if self.tokens[self.pos].kind != 'IDENT':
			return None
		tok = self.tokens[self.pos]
		self.pos += 1
		return tok

	def expect_ident(self):
		if self.tokens[self.pos].kind != 'IDENT':
			return None
		tok = self.tokens[self.pos]
		s = tok.str
		self.pos += 1
		return s

	def new_var(self, var):
		node = Node('VAR')
		node.var = var
		return node

	def find_var(self, tok):
		vl = self.locals
		while vl:
			var = vl.var
			if var.name == tok.str:
				return var
			vl = vl.next
		vl = self.globals
		while vl:
			var = vl.var
			if var.name == tok.str:
				return var
			vl = vl.next
		return None

	def push_var(self, name, ty, is_local):
		var = Var()
		var.name = name
		var.ty = ty
		var.is_local = is_local

		vl = VarList()
		vl.var = var
		if is_local:
			vl.next = self.locals
			self.locals =vl
		else:
			vl.next = self.globals
			self.globals = vl
		return var

	def read_expr_stmt(self):
		node =Node('EXPR_STMT')
		node.lhs = self.expr()
		return node

	def func_args(self):
		if self.consume(")"):
			return None
		node = self.assign()
		cur = node
		while self.consume(","):
			cur.next = self.assign()
			cur = cur.next
		self.expect(")")
		return node

	def read_func_params(self):
		if self.consume(")"):
			return None
		head = self.read_func_param()
		cur = head
		while not self.consume(")"):
			self.expect(",")
			cur.next = self.read_func_param()
			cur = cur.next
		return head

	def read_func_param(self):
		vl = VarList()
		ty = self.basetype()
		name = self.expect_ident()
		ty = self.read_type_suffix(ty)
		vl.var = self.push_var(name, ty, True)
		return vl

	def declaration(self):
		tok = self.tokens[self.pos]
		ty = self.basetype()
		name = self.expect_ident()
		ty = self.read_type_suffix(ty)
		var = self.push_var(name, ty, True)

		if self.consume(";"):
			return Node('NULL')

		self.expect("=")
		lhs = self.new_var(var)
		rhs = self.expr()
		self.expect(";")
		n = Node('ASSIGN', lhs=lhs, rhs=rhs)
		node = Node('EXPR_STMT')
		node.lhs = n
		return node

	def basetype(self):
		self.expect("int")
		ty = int_type()
		while self.consume("*"):
			ty = pointer_to(ty)
		return ty

	def read_type_suffix(self, base):
		if not self.consume("["):
			return base

		sz = self.expect_number()
		self.expect("]")
		base = self.read_type_suffix(base)
		return array_of(base, sz)

	def is_function(self):
		save = self.pos
		self.basetype()
		isfunc = (self.tokens[self.pos].kind == 'IDENT' and self.tokens[self.pos + 1].str == '(')
		self.pos = save
		return isfunc

	def global_var(self):
		ty = self.basetype()
		name = self.expect_ident()
		ty = self.read_type_suffix(ty)
		self.expect(";")
		self.push_var(name, ty, False)







	def function(self):
		self.locals = None
		fn = Function()
		self.basetype()
		fn.name = self.expect_ident()
		self.expect("(")
		fn.params = self.read_func_params()
		self.expect("{")
		head = Node('DUMMY')
		cur = head
		while not self.consume("}"):
			cur.next = self.stmt()
			cur = cur.next
		fn.node = head.next
		fn.locals = self.locals
		return fn

	def program(self):
		head = Function()
		head.next = None
		cur = head
		self.glovals = []

		while not self.at_eof():
			if self.is_function():
				cur.next = self.function()
				cur = cur.next
			else:
				self.global_var()

		prog = Program()
		prog.globals = self.globals
		prog.fns = head.next
		return prog

	def stmt(self):
		if self.consume("return"):
			node = Node('RETURN')
			node.lhs = self.expr()
			self.expect(";")
			return node

		if self.consume("if"):
			node = Node('IF')
			self.expect("(")
			node.cond = self.expr()
			self.expect(")")
			node.then = self.stmt()
			if self.consume("else"):
				node.els = self.stmt()
			return node

		if self.consume("while"):
			node = Node('WHILE')
			self.expect("(")
			node.cond = self.expr()
			self.expect(")")
			node.then = self.stmt()
			return node

		if self.consume("for"):
			node = Node('FOR')
			self.expect("(")
			if not self.consume(";"):
				node.init = self.read_expr_stmt()
				self.expect(";")
			if not self.consume(";"):
				node.cond = self.expr()
				self.expect(";")
			if not self.consume(")"):
				node.inc = self.read_expr_stmt()
				self.expect(")")
			node.then = self.stmt()
			return node

		if self.consume("{"):
			node = Node('BLOCK')
			cur = node
			while not self.consume("}"):
				cur.next = self.stmt()
				cur = cur.next
			node.body = node.next
			return node

		if self.peek("int"):
			return self.declaration()

		node = Node('EXPR_STMT')
		node.lhs = self.expr()
		self.expect(";")
		return node

	def expr(self):
		return self.assign()

	def assign(self):
		node = self.equality()
		if self.consume("="):
			node = Node('ASSIGN', lhs=node, rhs=self.assign())
		return node

	def equality(self):
		node = self.relational()
		while True:
			if self.consume("=="):
				node = Node('EQ', lhs=node, rhs=self.relational())
			elif self.consume("!="):
				node = Node('NE', lhs=node, rhs=self.relational())
			else:
				return node

	def relational(self):
		node = self.add()
		while True:
			if self.consume("<"):
				node = Node("LT", lhs=node, rhs=self.add())
			elif self.consume("<="):
				node = Node("LE", lhs=node, rhs=self.add())
			elif self.consume(">"):
				node = Node("LT", lhs=self.add(), rhs=node)
			elif self.consume(">="):
				node = Node("LE", self.add(), node)
			else:
				return node

	def add(self):
		node = self.mul()
		while True:
			if self.consume("+"):
				node = Node('ADD', lhs=node, rhs=self.mul())
			elif self.consume("-"):
				node = Node('SUB', lhs=node, rhs=self.mul())
			else:
				return node

	def mul(self):
		node = self.unary()
		while True:
			if self.consume("*"):
				node = Node('MUL', lhs=node, rhs=self.unary())
			elif self.consume("/"):
				node = Node('DIV', lhs=node, rhs=self.unary())
			else:
				return node

	def unary(self):
		if self.consume("+"):
			return self.unary()
		elif self.consume("-"):
			node = Node('SUB', lhs=self.new_num(0), rhs=self.unary())
			return node
		elif self.consume("&"):
			node = Node('ADDR', lhs=self.unary())
			return node
		elif self.consume("*"):
			node = Node('DEREF', lhs=self.unary())
			return node
		else:
			return self.postfix()

	def postfix(self):
		node = self.primary()

		while self.consume("["):
			exp = Node('ADD', lhs=node, rhs=self.expr())
			self.expect("]")
			node = Node('DEREF', lhs=exp)

		return node

	def primary(self):
		if self.consume("("):
			node = self.expr()
			self.expect(")")
			return node

		current_tok = self.current_peek()

		if tok:= self.consume("sizeof"):
			node = Node('SIZEOF', lhs=self.unary())
			return node

		if tok:= self.consume_ident():
			if self.consume("("):
				node = Node('FUNCALL')
				node.funcname = tok.str
				node.args = self.func_args()
				return node

			var = self.find_var(tok)
			if not var:
				raise Exception(f"undefined variable: {tok.str}")
			return self.new_var(var)

		if current_tok.kind == 'NUM':
			return Node('NUM', val=self.expect_number())

