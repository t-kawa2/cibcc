class Node:
	def __init__(self, kind, lhs=None, rhs=None, name=None, val=None, next=None):
		self.kind = kind
		self.lhs = lhs
		self.rhs = rhs
		self.name = name
		self.val = val
		self.next = next

class Parser:
	def __init__(self, tokens):
		self.tokens = tokens
		self.pos = 0

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

	def new_lvar(self, name):
		node = Node('IDENT')
		node.name = name
		return node




	def program(self):
		head = Node('DUMMY')
		cur = head

		while not self.at_eof():
			cur.next = self.stmt()
			cur = cur.next

		return head.next

	def stmt(self):
		if self.consume("return"):
			node = Node('RETURN')
			node.lhs = self.expr()
			self.expect(";")
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
			node = Node('LVAR')
			node.name = tok.str
			return node

		return Node('NUM', val=self.expect_number())

