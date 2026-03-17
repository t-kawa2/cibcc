import sys

class Token:
	def __init__(self, kind, val=None):
		self.kind = kind
		self.val = val

def tokenize(p):
	tokens = []
	p =p.replace(' ', '')
	i = 0
	while i < len(p):
		if p[i] in '+-*/()':
			tokens.append(Token('RESERVED', p[i]))
			i += 1
		elif p[i].isdigit():
			start = i;
			while i < len(p) and p[i].isdigit():
				i += 1
			tokens.append(Token('NUM', int(p[start:i])))
		else:
			raise Exception(f"invalid token: {p[i]}")
	tokens.append(Token('EOF'))
	return tokens

class Node:
	def __init__(self, kind, lhs=None, rhs=None, val=None):
		self.kind = kind
		self.lhs = lhs
		self.rhs = rhs
		self.val = val

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

	def expr(self):
		node = self.mul()
		while True:
			if self.consume('+'):
				node = Node('ADD', node, self.mul())
			elif self.consume('-'):
				node = Node('SUB', node, self.mul())
			else:
				return node

	def mul(self):
		node = self.unary()
		while True:
			if self.consume('*'):
				node = Node('MUL', node, self.unary())
			elif self.consume('/'):
				node = Node('DIV', node, self.unary())
			else:
				return node

	def unary(self):
		if self.consume('+'):
			return self.unary()
		elif self.consume('-'):
			node = Node('SUB', self.new_num(0), self.unary())
			return node
		else:
			return self.primary()

	def primary(self):
		if self.consume('('):
			node = self.expr()
			self.expect(')')
			return node

		return Node('NUM', val=self.expect_number())


def gen(node):
	if node.kind == 'NUM':
		print(f"  push {node.val}")
		return

	gen(node.lhs)
	gen(node.rhs)

	print("  pop rdi")
	print("  pop rax")

	if node.kind == 'ADD':
		print("  add rax, rdi")
	elif node.kind == 'SUB':
		print("  sub rax, rdi")
	elif node.kind == 'MUL':
		print("  imul rax, rdi")
	elif node.kind == 'DIV':
		print("  cqo")
		print("  idiv rdi")

	print("  push rax")


def main():
	if len(sys.argv) != 2:
		print("引数の個数が正しくありません")
		return

	input = sys.argv[1]
	tokens = tokenize(input)
	parser = Parser(tokens)
	node = parser.expr()

	print(".intel_syntax noprefix")
	print(".global main");
	print("main:")

	gen(node)

	print("  pop rax")
	print("  ret")

if __name__ == "__main__":
	main()

