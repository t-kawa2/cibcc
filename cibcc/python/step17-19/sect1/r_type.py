class Type:
	def __init__(self, kind=None, base=None):
		self.kind = kind
		self.base = base

def add_type(prog):
	fn = prog
	while fn:
		n = fn.node
		while n:
			visit(n)
			n = n.next
		fn = fn.next

def visit(node):
	if not node:
		return

	visit(node.lhs)
	visit(node.rhs)
	visit(node.cond)
	visit(node.then)
	visit(node.els)
	visit(node.init)
	visit(node.inc)

	n = node.body
	while n:
		visit(n)
		n = n.next
	n = node.args
	while n:
		visit(n)
		n = n.next

	match node.kind:
		case 'MUL' | 'DIV' | 'EQ' | 'NE' | 'LT' | 'LE' | 'VAR' | 'FUNCALL' | 'NUM':
			node.ty = int_type()
			return
		case 'ADD':
			if node.rhs.ty.kind == 'PTR':
				tmp = node.lhs
				node.lhs = node.rhs
				node.rhs = tmp
			if node.rhs.ty.kind == 'PTR':
				print("invalid pointer arthimetic operands")
			node.ty = node.lhs.ty
			return
		case 'SUB':
			if node.rhs.ty.kind == 'PTR':
				print("invalid pointer arthimetic operands")
			node.ty = node.lhs.ty
			return
		case 'ASSIGN':
			node.ty = node.lhs.ty
			return
		case 'ADDR':
			node.ty = pointer_to(node.lhs.ty)
			return
		case 'DEREF':
			if node.lhs.ty.kind == 'PTR':
				node.ty = node.lhs.ty.base
			else:
				node.ty = int_type()
			return

def int_type():
	ty = Type('INT')
	return ty

def pointer_to(base):
	ty = Type('PTR')
	ty.base = base
	return ty


