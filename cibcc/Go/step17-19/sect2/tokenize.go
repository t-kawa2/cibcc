package main

import (
	"reflect"
)

type TokenKind	int

const (
	TK_RESERVED TokenKind = iota
	TK_IDENT
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

func isAlnum(c rune) bool {
	return isAlpha(c) || isDigit(c)
}

func isAlpha(c rune) bool {
	return ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_'
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
		case ';':	fallthrough
		case '=':	fallthrough
		case '{':	fallthrough
		case '}':	fallthrough
		case ',':	fallthrough
		case '&':	return true
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

func startswith(str []rune, op []rune) bool {
	if len(str) < len(op) {
		return false
	}
	return reflect.DeepEqual(str[0:len(op)], op)
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
		if startswith(p, []rune("return")) && !isAlnum(p[6]) {
			cur = new_Token(TK_RESERVED, cur, p, 6)
			p = p[6:]
			continue
		}
		if startswith(p, []rune("if")) {
			cur = new_Token(TK_RESERVED, cur, p, 2)
			p = p[2:]
			continue
		}
		if startswith(p, []rune("else")) {
			cur = new_Token(TK_RESERVED, cur, p, 4)
			p = p[4:]
			continue
		}
		if startswith(p, []rune("while")) {
			cur = new_Token(TK_RESERVED, cur, p, 5)
			p = p[5:]
			continue
		}
		if startswith(p, []rune("for")) {
			cur = new_Token(TK_RESERVED, cur, p, 3)
			p = p[3:]
			continue
		}
		if startswith(p, []rune("int")) {
			cur = new_Token(TK_RESERVED, cur, p, 3)
			p = p[3:]
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
		if isAlpha(p[0]) {
			q := p
			r := len(p)
			p = p[1:]

			for len(p) >0 && isAlnum(p[0]) {
				p = p[1:]
			}
			cur = new_Token(TK_IDENT, cur, q, r-len(p))
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

