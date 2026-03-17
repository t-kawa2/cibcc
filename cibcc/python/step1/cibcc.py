import sys

args = sys.argv
if len(args) != 2:
	print("引数の個数が正しくありません")
	sys.exit()

input = sys.argv[1]

print(".intel_syntax noprefix")
print(".global main");
print("main:")

print("  mov rax, ", input)

print("  ret")

