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
	ND_ADDR
	ND_DEREF
	ND_RETURN
	ND_IF
	ND_WHILE
	ND_FOR
	ND_BLOCK
	ND_FUNCALL
	ND_EXPR_STMT
	ND_VAR
	ND_NUM
)

type Node struct {
	kind NodeKind
	next *Node
	lhs *Node
	rhs *Node
	cond *Node
	then *Node
	els *Node
	init *Node
	inc *Node
	body *Node
	funcname []rune
	args *Node
	va *Var
	val int
}

type Var struct {
	next *Var
	va *Var
	name []rune
	offset int
}

type VarList struct {
	next *VarList
	va *Var
}

type Program struct {
	fns *Function
}

type Function struct {
	next *Function
	name []rune
	node *Node
	params *VarList
	locals *VarList
	stack_size int
}

var node *Node
var locals *VarList

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
	for vl := locals; vl != nil; vl = vl.next {
		va := vl.va
		if len(va.name) == len(toks) && reflect.DeepEqual(toks, va.name) {
			return va
		}
	}
	return nil
}

func push_var(name []rune) *Var {
	va := &Var{name: name}
	vl := &VarList{va: va, next: locals}
	locals = vl
	return va
}

func read_expr_stmt() *Node {
	return new_unary(ND_EXPR_STMT, expr())
}

func func_args() *Node {
	if consume([]rune(")")) != nil {
		return nil
	}

	h := assign()
	cur := h
	for consume([]rune(",")) != nil {
		cur.next = assign()
		cur = cur.next
	}
	expect([]rune(")"))
	return h
}

func read_func_params() *VarList {
	if consume([]rune(")")) != nil {
		return nil
	}

	head := read_func_param()
	cur := head

	for consume([]rune(")")) == nil {
		expect([]rune(","))
		cur.next = read_func_param()
		cur = cur.next
	}
	return head
}

func read_func_param() *VarList {
	name := expect_ident()
	return &VarList{va: push_var(name)}
}







func program() *Program {

	var head Function
	head.next = nil
	cur := &head

	for !at_eof() {
		cur.next = function()
		cur = cur.next
	}
	return &Program{fns: head.next}
}

func function() *Function {
	locals = nil

	name := expect_ident()
	expect([]rune("("))
	params := read_func_params()
	expect([]rune("{"))

	var head Node
	head.next = nil
	cur := &head

	for consume([]rune("}")) == nil {
		cur.next = stmt()
		cur = cur.next
	}
	return &Function{node: head.next, locals: locals, name: name, params: params}
}

func stmt() *Node {
	if consume([]rune("return")) != nil {
		node := new_unary(ND_RETURN, expr())
		expect([]rune(";"))
		return node
	}
	if consume([]rune("if")) != nil {
		node := &Node{kind: ND_IF}
		expect([]rune("("))
		node.cond = expr()
		expect([]rune(")"))
		node.then = stmt()
		if consume([]rune("else")) != nil {
			node.els = stmt()
		}
		return node
	}
	if consume([]rune("while")) != nil {
		node := &Node{kind: ND_WHILE}
		expect([]rune("("))
		node.cond = expr()
		expect([]rune(")"))
		node.then = stmt()
		return node
	}
	if consume([]rune("for")) != nil {
		node := &Node{kind: ND_FOR}
		expect([]rune("("))
		if consume([]rune(";")) == nil {
			node.init = read_expr_stmt()
			expect([]rune(";"))
		}
		if consume([]rune(";")) == nil {
			node.cond = expr()
			expect([]rune(";"))
		}
		if consume([]rune(")")) == nil {
			node.inc = read_expr_stmt()
			expect([]rune(")"))
		}
		node.then = stmt()
		return node
	}
	if consume([]rune("{")) != nil {
		var head Node
		head.next = nil
		cur := &head

		for consume([]rune("}")) == nil {
			cur.next = stmt()
			cur = cur.next
		}

		node := &Node{kind: ND_BLOCK}
		node.body = head.next
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
	if consume([]rune("&")) != nil {
		return new_unary(ND_ADDR, unary())
	}
	if consume([]rune("*")) != nil {
		return new_unary(ND_DEREF, unary())
	}
	return primary()
}

func primary() *Node {
	if consume([]rune("(")) != nil {
		node := expr()
		expect([]rune(")"))
		return node
	}

	tt := token
	tok := consume_ident()
	if tok != nil {
		if consume([]rune("(")) != nil {
			args := func_args()
			return &Node{kind: ND_FUNCALL, funcname: tok.str[:tok.len], args: args}
		}
	}

	token = tt
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

