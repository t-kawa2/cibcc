class Token:
	def __init__(self, kind, val=None, str=None, contents=None, cont_len=None):
		self.kind = kind
		self.val = val
		self.str = str
		self.contents = contents
		self.cont_len = cont_len

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
		if p[i:i+6] == "sizeof":
			tokens.append(Token('RESERVED', str=p[i:i+6]))
			i += 6
			continue
		if p[i:i+4] == "char":
			tokens.append(Token('RESERVED', str=p[i:i+4]))
			i += 4
			continue
		if p[i:i+2] == "==" or p[i:i+2] == "!=" or p[i:i+2] == "<=" or p[i:i+2] == ">=":
			tokens.append(Token('RESERVED', str=p[i:i+2]))
			i += 2
			continue
		if p[i] in '+-*/()<>;={},&[]':
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
		if p[i] == '"':
			start = i
			i += 1

			if p[i] == '\\':
				i += 1
				string_literal = get_escape_char(p[i])
				i += 1
				if p[i] == '"':
					tokens.append(Token('STR', contents=string_literal, cont_len=len(string_literal)))
					i += 1
					continue
			else:
				while i <len(p) and p[i] != '"':
					i += 1

				res_contents = list(p[start+1:i])
				res_contents.append('\0')

				tokens.append(Token('STR', contents=res_contents, cont_len=len(res_contents)))
				i += 1
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

def get_escape_char(c):
	match c:
		case 'a':	return '\a'
		case 'b':	return '\b'
		case 't':	return '\t'
		case 'n':	return '\n'
		case 'v':	return '\v'
		case 'f':	return '\f'
		case 'r':	return '\r'
		case 'e':	return chr(27)
		case '0':	return '\0'
		case   _:	return c
