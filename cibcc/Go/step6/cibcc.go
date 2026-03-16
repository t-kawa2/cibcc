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
		case '-':	fallthrough
		case '*':	fallthrough
		case '/':	fallthrough
		case '(':	fallthrough
		case ')':	return true
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

type NodeKind int

const (
	ND_ADD	NodeKind = iota
	ND_SUB
	ND_MUL
	ND_DIV
	ND_NUM
)

type Node struct {
	kind NodeKind
	next *Node
	lhs *Node
	rhs *Node
	val int
}

var node *Node

func new_Node(kind NodeKind, lhs *Node, rhs *Node) *Node {
	return &Node{kind: kind, lhs: lhs, rhs: rhs}
}

func new_number(val int) *Node {
	return &Node{kind: ND_NUM, val: val}
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


func expr() *Node {
	node := mul()
	for {
		if consume('+') {
			node = new_Node(ND_ADD, node, mul())
		} else if consume('-') {
			node = new_Node(ND_SUB, node, mul())
		} else {
			return node
		}
	}
}

func mul() *Node {
	node := unary()
	for {
		if consume('*') {
			node = new_Node(ND_MUL, node, unary())
		} else if consume('/') {
			node = new_Node(ND_DIV, node, unary())
		} else {
			return node
		}
	}
}

func unary() *Node {
	if consume('+') {
		return unary()
	}
	if consume('-') {
		return new_Node(ND_SUB, new_number(0), unary())
	}
	return primary()
}

func primary() *Node {
	if consume('(') {
		node := expr()
		expect(')')
		return node
	}

	return new_number(expect_number())
}

func gen(node *Node) {
	if node.kind == ND_NUM {
		fmt.Printf("  push %d\n", node.val)
		return
	}

	gen(node.lhs)
	gen(node.rhs)

	fmt.Println("  pop rdi")
	fmt.Println("  pop rax")

	switch node.kind {
		case ND_ADD:
			fmt.Println("  add rax, rdi")
		case ND_SUB:
			fmt.Println("  sub rax, rdi")
		case ND_MUL:
			fmt.Println("  imul rax, rdi")
		case ND_DIV:
			fmt.Println("  cqo")
			fmt.Println("  idiv rdi")
	}

	fmt.Println("  push rax")
}

func main() {
	if len(os.Args) != 2 {
		fmt.Println("引数の個数が正しくありません")
	}

	token = tokenize([]rune(os.Args[1]))
	node = expr()

	fmt.Println(".intel_syntax noprefix")
	fmt.Println(".global main")
	fmt.Println("main:")

	fmt.Println("  mov rax, ", expect_number())

	gen(node)

	fmt.Println("  pop rax")
	fmt.Println("  ret")
}

