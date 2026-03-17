class Type:
	def __init__(self, kind=None, base=None, array_size=None):
		self.kind = kind
		self.base = base
		self.array_size = array_size

def add_type(prog):
	fn = prog.fns
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
		case 'MUL' | 'DIV' | 'EQ' | 'NE' | 'LT' | 'LE' | 'FUNCALL' | 'NUM':
			node.ty = int_type()
			return
		case 'VAR':
			node.ty = node.var.ty
			return
		case 'ADD':
			if node.rhs.ty.base:
				tmp = node.lhs
				node.lhs = node.rhs
				node.rhs = tmp
			if node.rhs.ty.base:
				print("invalid pointer arthimetic operands")
			node.ty = node.lhs.ty
			return
		case 'SUB':
			if node.rhs.ty.base:
				print("invalid pointer arthimetic operands")
			node.ty = node.lhs.ty
			return
		case 'ASSIGN':
			node.ty = node.lhs.ty
			return
		case 'ADDR':
			if node.lhs.ty.kind == 'ARRAY':
				node.ty = pointer_to(node.lhs.ty.base)
			else:
				node.ty = pointer_to(node.lhs.ty)
			return
		case 'DEREF':
			if not node.lhs.ty.base:
				print("invalid pointer dereference")
			node.ty = node.lhs.ty.base
			return
		case 'SIZEOF':
			node.kind = 'NUM'
			node.ty = int_type()
			node.val = size_of(node.lhs.ty)
			node.lhs = None
			return
		case 'STMT_EXPR':
			last = node.body
			while last.next:
				last = last.next
			node.ty = last.ty
			return

def int_type():
	ty = Type('INT')
	return ty

def pointer_to(base):
	ty = Type('PTR')
	ty.base = base
	return ty

def array_of(base, size):
	ty = Type()
	ty.kind = 'ARRAY'
	ty.base = base
	ty.array_size = size
	return ty

def size_of(ty):
	if ty.kind == 'CHAR':
		return 1
	elif ty.kind == 'INT' or ty.kind == 'PTR':
		return 8
	assert(ty.kind == 'ARRAY')
	return size_of(ty.base) * ty.array_size

def char_type():
	ty = Type('CHAR')
	return ty
