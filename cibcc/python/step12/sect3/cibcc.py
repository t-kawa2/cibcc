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

	offset = 0
	var = prog.locals
	while var:
		offset += 8
		var.offset = offset
		var = var.next

	prog.stack_size = offset

	codegen(prog)

if __name__ == "__main__":
	main()

