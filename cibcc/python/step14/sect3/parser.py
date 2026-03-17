class Node:
	def __init__(self, kind, lhs=None, rhs=None, cond=None, then=None, els=None, init=None, inc=None, body=None, funcname=None, val=None, next=None, var=None):
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
		self.val = val
		self.next = next
		self.var = var

class Var:
	def __init__(self, next=None, name=None, offset=None):
		self.next = next
		self.name = name
		self.offset = offset

class Program:
	def __init__(self, node=None, locals=None, stack_size=None):
		self.node = node
		self.locals = locals
		self.stack_size = stack_size

class Parser:
	def __init__(self, tokens):
		self.tokens = tokens
		self.pos = 0
		self.locals = None

	def consume(self, op):
		if self.tokens[self.pos].kind != 'RESERVED' or self.tokens[self.pos].val != op:
			return False
		self.pos += 1
		return True

	def expect(self, op):
		if not self.consume(op):
			raise Exception(f"expected {op}")

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

	def new_var(self, var):
		node = Node('VAR')
		node.var = var
		return node

	def find_var(self, tok):
		var = self.locals
		while var:
			if var.name == tok.str:
				return var
			var = var.next
		return None

	def push_var(self, name):
		var = Var(next=self.locals, name=name)
		self.locals =var
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





	def program(self):
		head = Node('DUMMY')
		cur = head

		while not self.at_eof():
			cur.next = self.stmt()
			cur = cur.next

		return Program(node=head.next, locals=self.locals)

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
				node = Node('EQ', node, self.relational())
			elif self.consume("!="):
				node = Node('NE', node, self.relational())
			else:
				return node

	def relational(self):
		node = self.add()
		while True:
			if self.consume("<"):
				node = Node("LT", node, self.add())
			elif self.consume("<="):
				node = Node("LE", node, self.add())
			elif self.consume(">"):
				node = Node("LT", self.add(), node)
			elif self.consume(">="):
				node = Node("LE", self.add(), node)
			else:
				return node

	def add(self):
		node = self.mul()
		while True:
			if self.consume("+"):
				node = Node('ADD', node, self.mul())
			elif self.consume("-"):
				node = Node('SUB', node, self.mul())
			else:
				return node

	def mul(self):
		node = self.unary()
		while True:
			if self.consume("*"):
				node = Node('MUL', node, self.unary())
			elif self.consume("/"):
				node = Node('DIV', node, self.unary())
			else:
				return node

	def unary(self):
		if self.consume("+"):
			return self.unary()
		elif self.consume("-"):
			node = Node('SUB', self.new_num(0), self.unary())
			return node
		else:
			return self.primary()

	def primary(self):
		if self.consume("("):
			node = self.expr()
			self.expect(")")
			return node

		tok = self.consume_ident()
		if tok:
			if self.consume("("):
				node = Node('FUNCALL')
				node.funcname = tok.str
				node.args = self.func_args()
				return node

			var = self.find_var(tok)
			if not var:
				var = self.push_var(tok.str)
			return self.new_var(var)

		return Node('NUM', val=self.expect_number())

