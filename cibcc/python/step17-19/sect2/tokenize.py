class Token:
	def __init__(self, kind, val=None, str=None):
		self.kind = kind
		self.val = val
		self.str = str

def tokenize(p):
	tokens = []
	i = 0
	while i < len(p):
		if p[i].isspace():
			i += 1
			continue
		if p[i:i+6] == "return":
			tokens.append(Token('RESERVED', str=p[i:i+6]))
			i += 6
			continue
		if p[i:i+2] == "if":
			tokens.append(Token('RESERVED', str=p[i:i+2]))
			i += 2
			continue
		if p[i:i+4] == "else":
			tokens.append(Token('RESERVED', str=p[i:i+4]))
			i += 4
			continue
		if p[i:i+5] == "while":
			tokens.append(Token('RESERVED', str=p[i:i+5]))
			i += 5
			continue
		if p[i:i+3] == "for":
			tokens.append(Token('RESERVED', str=p[i:i+3]))
			i += 3
		if p[i:i+3] == "int":
			tokens.append(Token('RESERVED', str=p[i:i+3]))
			i += 3
			continue
		if p[i:i+2] == "==" or p[i:i+2] == "!=" or p[i:i+2] == "<=" or p[i:i+2] == ">=":
			tokens.append(Token('RESERVED', str=p[i:i+2]))
			i += 2
			continue
		if p[i] in '+-*/()<>;={},&':
			tokens.append(Token('RESERVED', str=p[i]))
			i += 1
			continue
		if p[i].isalpha() or p[i] == '_':
			start = i
			i += 1
			while i < len(p) and (p[i].isalnum() or p[i] == '_'):
				i += 1
			tokens.append(Token('IDENT', str=p[start:i]))
			continue
		if p[i].isdigit():
			start = i
			while i < len(p) and p[i].isdigit():
				i += 1
			tokens.append(Token('NUM', val=int(p[start:i]), str=p[start:i]))
			continue
		else:
			raise Exception(f"invalid token: {p[i]}")
	tokens.append(Token('EOF'))
	return tokens

