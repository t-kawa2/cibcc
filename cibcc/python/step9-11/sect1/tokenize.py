class Token:
	def __init__(self, kind, val=None):
		self.kind = kind
		self.val = val

def tokenize(p):
	tokens = []
	p =p.replace(' ', '')
	i = 0
	while i < len(p):
		if p[i:i+2] == "==" or p[i:i+2] == "!=" or p[i:i+2] == "<=" or p[i:i+2] == ">=":
			tokens.append(Token('RESERVED', p[i:i+2]))
			i += 2
		elif p[i] in '+-*/()<>;':
			tokens.append(Token('RESERVED', p[i]))
			i += 1
		elif p[i].isdigit():
			start = i
			while i < len(p) and p[i].isdigit():
				i += 1
			tokens.append(Token('NUM', int(p[start:i])))
		else:
			raise Exception(f"invalid token: {p[i]}")
	tokens.append(Token('EOF'))
	return tokens

