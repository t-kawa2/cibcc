import sys
from tokenize import *
from parser import *
from codegen import *
from r_type import *

def main():
	if len(sys.argv) != 2:
		print("引数の個数が正しくありません")
		return

	input = sys.argv[1]
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

		fn.stack_size = (offset + 15) // 16 * 16
		fn = fn.next

	codegen(prog)

if __name__ == "__main__":
	main()

