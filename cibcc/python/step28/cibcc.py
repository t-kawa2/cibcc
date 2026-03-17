import sys
from tokenize import *
from parser import *
from codegen import *
from r_type import *

def align_to(n, align):
	return (n + align - 1) & ~(align - 1)

def main():
	path = './tests'
	with open(path) as f:
		input = f.read()

	tokens = tokenize(input)
	parser = Parser(tokens)
	prog = parser.program()
	add_type(prog)

	fn = prog.fns
	while fn:
		offset = 0
		vl = fn.locals
		while vl:
			var = vl.var
			offset += size_of(var.ty)
			var.offset = offset
			vl = vl.next

		fn.stack_size = align_to(offset, 8)
		fn = fn.next

	codegen(prog)

if __name__ == "__main__":
	main()

