import sys
from tokenize import *
from parser import *
from codegen import *

def main():
	if len(sys.argv) != 2:
		print("引数の個数が正しくありません")
		return

	input = sys.argv[1]
	tokens = tokenize(input)
	parser = Parser(tokens)
	prog = parser.program()

	fn = prog
	while fn:
		offset = 0
		var = fn.locals
		while var:
			offset += 8
			var.offset = offset
			var = var.next

		fn.stack_size = (offset + 15) // 16 * 16
		fn = fn.next

	codegen(prog)

if __name__ == "__main__":
	main()

