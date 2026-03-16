package main

import (
	"os"
	"fmt"
	"reflect"
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
		case '<':	return true
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

type NodeKind int

const (
	ND_ADD	NodeKind = iota
	ND_SUB
	ND_MUL
	ND_DIV
	ND_EQ
	ND_NE
	ND_LT
	ND_LE
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

func peek(s []rune) bool {
	if token.kind != TK_RESERVED {
		return false
	}
	if len(s) != token.len {
		return false
	}
	if !reflect.DeepEqual(token.str[:token.len], s) {
		return false
	}
	return true
}

func consume(op []rune) *Token {
	if !peek(op) {
		return nil
	}
	tok := token
	token = token.next
	return tok
}

func expect(op []rune) {
	if !peek(op) {
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
	return equality()
}

func equality() *Node {
	node := relational()
	for {
		if consume([]rune("==")) != nil {
			node = new_Node(ND_EQ, node, relational())
		} else if consume([]rune("!=")) != nil {
			node = new_Node(ND_NE, node, relational())
		} else {
			return node
		}
	}
}

func relational() *Node {
	node := add()
	for {
		if consume([]rune("<")) != nil {
			node = new_Node(ND_LT, node, add())
		} else if consume([]rune("<=")) != nil {
			node = new_Node(ND_LE, node, add())
		} else if consume([]rune(">")) != nil {
			node = new_Node(ND_LT, add(), node)
		} else if consume([]rune(">=")) != nil {
			node = new_Node(ND_LE, add(), node)
		} else {
			return node
		}
	}
}

func add() *Node {
	node := mul()
	for {
		if consume([]rune("+")) != nil {
			node = new_Node(ND_ADD, node, mul())
		} else if consume([]rune("-")) != nil {
			node = new_Node(ND_SUB, node, mul())
		} else {
			return node
		}
	}
}

func mul() *Node {
	node := unary()
	for {
		if consume([]rune("*")) != nil {
			node = new_Node(ND_MUL, node, unary())
		} else if consume([]rune("/")) != nil {
			node = new_Node(ND_DIV, node, unary())
		} else {
			return node
		}
	}
}

func unary() *Node {
	if consume([]rune("+")) != nil {
		return unary()
	}
	if consume([]rune("-")) != nil {
		return new_Node(ND_SUB, new_number(0), unary())
	}
	return primary()
}

func primary() *Node {
	if consume([]rune("(")) != nil {
		node := expr()
		expect([]rune(")"))
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
		case ND_EQ:
			fmt.Println("  cmp rax, rdi")
			fmt.Println("  sete al");
			fmt.Println("  movzb rax, al")
		case ND_NE:
			fmt.Println("  cmp rax, rdi")
			fmt.Println("  setne al");
			fmt.Println("  movzb rax, al")
		case ND_LT:
			fmt.Println("  cmp rax, rdi")
			fmt.Println("  setl al");
			fmt.Println("  movzb rax, al")
		case ND_LE:
			fmt.Println("  cmp rax, rdi")
			fmt.Println("  setle al");
			fmt.Println("  movzb rax, al")
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

	gen(node)

	fmt.Println("  pop rax")
	fmt.Println("  ret")
}

