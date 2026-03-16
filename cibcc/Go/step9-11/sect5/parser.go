package main

import (
	"fmt"
	"reflect"
)

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
	ND_ASSIGN
	ND_RETURN
	ND_EXPR_STMT
	ND_VAR
	ND_NUM
)

type Node struct {
	kind NodeKind
	next *Node
	lhs *Node
	rhs *Node
	va *Var
	val int
}

type Var struct {
	next *Var
	name []rune
	offset int
}

type Program struct {
	node *Node
	locals *Var
	stack_size int
}

var node *Node
var locals *Var

func new_Node(kind NodeKind, lhs *Node, rhs *Node) *Node {
	return &Node{kind: kind, lhs: lhs, rhs: rhs}
}

func new_unary(kind NodeKind, expr *Node) *Node {
	return &Node{kind: kind, lhs: expr}
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

func consume_ident() *Token {
	if token.kind != TK_IDENT {
		return nil
	}
	tok := token
	token = token.next
	return tok
}

func expect_ident() []rune {
	if token.kind != TK_IDENT {
		return nil
	}
	s := token.str[:token.len]
	token = token.next
	return s
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

func new_var(va *Var) *Node {
	return &Node{kind: ND_VAR, va: va}
}

func find_var(toks []rune) *Var {
	for va := locals; va != nil; va = va.next {
		if len(va.name) == len(toks) && reflect.DeepEqual(toks, va.name) {
			return va
		}
	}
	return nil
}

func push_var(name []rune) *Var {
	va := &Var{name: name}
	va.next = locals
	locals = va
	return va
}





func program() *Program {
	locals = nil

	var head Node
	head.next = nil
	cur := &head

	for !at_eof() {
		cur.next = stmt()
		cur = cur.next
	}
	return &Program{node: head.next, locals: locals}
}

func stmt() *Node {
	if consume([]rune("return")) != nil {
		node := new_unary(ND_RETURN, expr())
		expect([]rune(";"))
		return node
	}
	node := new_unary(ND_EXPR_STMT, expr())
	expect([]rune(";"))
	return node
}

func expr() *Node {
	return assign()
}

func assign() *Node {
	node := equality()

	if consume([]rune("=")) != nil {
		node = new_Node(ND_ASSIGN, node, assign())
	}

	return node
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

	toks := expect_ident()
	if toks != nil {
		va := find_var(toks)
		if va == nil {
			va = push_var(toks)
		}
		return new_var(va)
	}

	return new_number(expect_number())
}

