import sys

class Token:
	def __init__(self, kind, val=None):
		self.kind = kind
		self.val = val

def tokenize(s):

	s = s.strip()

	tokens = []
	pos = 0
	lenstr = len(s)

	while pos < lenstr:
		c = s[pos]
		if pos == 0:
			c0 = '#'
		else:
			c0 = s[pos-1]
		pos += 1

		if c == '+' or c == '-':
			tokens.append(Token('RESERVED', c))
			continue

		if c.isdigit() == True:
			tok = c
			while pos < lenstr:
				c1 = s[pos]
				if c1.isdigit() != True:
					break;
				else:
					pos += 1
					tok = tok + c1

			tokens.append(Token('NUM', tok))

	return tokens


args = sys.argv
if len(args) != 2:
	print("引数の個数が正しくありません")
	sys.exit()

input = sys.argv[1]
tokens = tokenize(input)

print(".intel_syntax noprefix")
print(".global main");
print("main:")

print("  mov rax, ", tokens[0].val)
tokens = tokens[1:]

for tok in tokens:
	if tok.val == '+':
		print("  add rax, ", tokens[1].val)
		tokens = tokens[2:]
	elif tok.val == '-':
		print("  sub rax, ", tokens[1].val)
		tokens = tokens[2:]

print("  ret")

