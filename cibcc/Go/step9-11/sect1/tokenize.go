package main

type TokenKind	int

const (
	TK_RESERVED TokenKind = iota
	TK_NUM
	TK_EOF
)

type Token struct {
	kind TokenKind
	next *Token
	val int
	str []rune
	len int
}

var token *Token

func isSpace(c rune) bool {
	return c == ' ' || c == '\t' || c == '\n' || c ==  '\r'
}

func isDigit(c rune) bool {
	return '0' <= c && c <= '9'
}

func isReserved(c rune) bool {
	switch c {
		case '+':	fallthrough
		case '-':	fallthrough
		case '*':	fallthrough
		case '/':	fallthrough
		case '(':	fallthrough
		case ')':	fallthrough
		case '>':	fallthrough
		case '<':	fallthrough
		case ';':	return true
		default:	return false
	}
}

func strtoi(p *[]rune) int {
	s := *p
	c :=s[0]
	s = s[1:]

	acc := 0
	for {
		k := int(c - '0')
		acc *= 10
		acc += k
		if len(s) == 0 || !isDigit(s[0]) {
			break
		}
		c = s[0]
		s = s[1:]
	}
	*p = s
	return acc
}

func new_Token(kind TokenKind, cur *Token, str []rune, len int) *Token {
	p := &Token{kind: kind, str: str, len: len}
	cur.next = p
	return p
}

func tokenize(p []rune) *Token {
	var head Token
	head.next = nil
	cur := &head

	for len(p) > 0 {
		if isSpace(p[0]) {
			p = p[1:]
			continue
		}
		if p[0] == '=' && p[1] == '=' {
			cur = new_Token(TK_RESERVED, cur, p, 2)
			p = p[2:]
			continue
		}
		if p[0] == '!' && p[1] == '=' {
			cur = new_Token(TK_RESERVED, cur, p, 2)
			p = p[2:]
			continue
		}
		if p[0] == '<' && p[1] == '=' {
			cur = new_Token(TK_RESERVED, cur, p, 2)
			p = p[2:]
			continue
		}
		if p[0] == '>' && p[1] == '=' {
			cur = new_Token(TK_RESERVED, cur, p, 2)
			p = p[2:]
			continue
		}
		if isReserved(p[0]) {
			cur = new_Token(TK_RESERVED, cur, p, 1) 
			p = p[1:]
			continue
		}
		if isDigit(p[0]) {
			cur = new_Token(TK_NUM, cur, p, 0)
			v := strtoi(&p)
			cur.val = v
			continue
		}
	}
	new_Token(TK_EOF, cur, p, 0)
	return head.next
}

