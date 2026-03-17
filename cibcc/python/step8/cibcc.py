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
	node = parser.expr()

	print(".intel_syntax noprefix")
	print(".global main");
	print("main:")

	gen(node)

	print("  pop rax")
	print("  ret")

if __name__ == "__main__":
	main()

