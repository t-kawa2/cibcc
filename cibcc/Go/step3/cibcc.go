package main

import (
	"os"
	"fmt"
)

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
		case '-':	return true
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

func new_Token(kind TokenKind, cur *Token, str []rune) *Token {
	p := &Token{kind: kind, str: str}
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
		if isReserved(p[0]) {
			cur = new_Token(TK_RESERVED, cur, p) 
			p = p[1:]
			continue
		}
		if isDigit(p[0]) {
			cur = new_Token(TK_NUM, cur, p)
			v := strtoi(&p)
			cur.val = v
			continue
		}
	}
	new_Token(TK_EOF, cur, p)
	return head.next
}

func consume(op rune) bool {
	if token.kind != TK_RESERVED || token.str[0] != op {
		return false
	}
	token = token.next
	return true
}

func expect(op rune) {
	if token.kind != TK_RESERVED || token.str[0] != op {
		fmt.Println("not expected code")
	}
	token = token.next
}

func expect_number() int {
	v := token.val
	token = token.next
	return v
}

func at_eof() bool {
	return token.kind == TK_EOF
}


func main() {
	if len(os.Args) != 2 {
		fmt.Println("引数の個数が正しくありません")
	}

	token = tokenize([]rune(os.Args[1]))

	fmt.Println(".intel_syntax noprefix")
	fmt.Println(".global main")
	fmt.Println("main:")

	fmt.Println("  mov rax, ", expect_number())

	for !at_eof() {
		if consume('+') {
			fmt.Printf("  add rax, %d\n", expect_number())
		} else if consume('-') {
			fmt.Printf("  sub rax, %d\n", expect_number())
		}
	}
	fmt.Println("  ret")
}

