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
	ND_SIZEOF
	ND_BLOCK
	ND_FUNCALL
	ND_EXPR_STMT
	ND_STMT_EXPR
	ND_VAR
	ND_NUM
	ND_NULL
)

type Node struct {
	kind NodeKind
	next *Node
	ty *Type
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
	ty *Type
	islocal bool
	name []rune
	offset int
	contents []rune
	cont_len int
}

type VarList struct {
	next *VarList
	va *Var
}

type Program struct {
	globals *VarList
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
var globals *VarList
var labelcnt = 0

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

func find_var(tok *Token) *Var {
	for vl := locals; vl != nil; vl = vl.next {
		va := vl.va
		if len(va.name) == tok.len && reflect.DeepEqual(tok.str[:tok.len], va.name) {
			return va
		}
	}
	for vl := globals; vl != nil; vl = vl.next {
		va := vl.va
		if len(va.name) == tok.len && reflect.DeepEqual(tok.str[:tok.len], va.name) {
			return va
		}
	}
	return nil
}

func push_var(name []rune, ty *Type, islocal bool) *Var {
	va := &Var{name: name, ty: ty, islocal: islocal}
	vl := &VarList{va: va}
	if islocal {
		vl.next = locals
		locals = vl
	} else {
		vl.next = globals
		globals = vl
	}
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
	ty := basetype()
	name := expect_ident()
	ty = read_type_suffix(ty)
	return &VarList{va: push_var(name, ty, true)}
}

func declaration() *Node {
	ty := basetype()
	name := expect_ident()
	ty = read_type_suffix(ty)
	va := push_var(name, ty, true)

	if consume([]rune(";")) != nil {
		return &Node{kind: ND_NULL}
	}

	expect([]rune("="))
	lhs := new_var(va)
	rhs := expr()
	expect([]rune(";"))
	node :=new_Node(ND_ASSIGN, lhs, rhs)
	return new_unary(ND_EXPR_STMT, node)
}


func basetype() *Type {
	var ty *Type
	if consume([]rune("char")) != nil {
		ty = char_type()
	} else {
		expect([]rune("int"))
		ty = int_type()
	}
	for consume([]rune("*")) != nil {
		ty = pointer_to(ty)
	}
	return ty
}

func read_type_suffix(base *Type) *Type {
	if consume([]rune("[")) == nil {
		return base
	}
	sz := expect_number()
	expect([]rune("]"))
	base = read_type_suffix(base)
	return array_of(base, sz)
}

func is_function() bool {
	tok := token
	basetype()
	isfunc := consume_ident() != nil && consume([]rune("(")) != nil
	token = tok
	return isfunc
}

func global_var() {
	ty := basetype()
	name := expect_ident()
	ty = read_type_suffix(ty)
	expect([]rune(";"))
	push_var(name, ty, false)
}

func is_typename() bool {
	return peek([]rune("char")) || peek([]rune("int"))
}

func new_label() []rune {
	s := fmt.Sprintf(".L.data.%d", labelcnt)
	labelcnt += 1
	return []rune(s)
}

func stmt_expr(tok *Token) *Node {
	node := &Node{kind: ND_STMT_EXPR}
	node.body = stmt()
	cur := node.body

	for consume([]rune("}")) == nil {
		cur.next = stmt()
		cur = cur.next
	}
	expect([]rune(")"))

	if cur.kind != ND_EXPR_STMT {
		fmt.Println("stmt expr returning void is not supported")
	}
	cur = cur.lhs
	return node
}






func program() *Program {
	globals = nil

	var head Function
	head.next = nil
	cur := &head

	for !at_eof() {
		if is_function() {
			cur.next = function()
			cur = cur.next
		} else {
			global_var()
		}
	}
	return &Program{fns: head.next, globals: globals}
}

func function() *Function {
	locals = nil

	basetype()
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
	if is_typename() {
		return declaration()
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
	return postfix()
}

func postfix() *Node {
	node := primary()
	for tok := consume([]rune("[")); tok != nil; tok = consume([]rune("[")) {
		exp := new_Node(ND_ADD, node, expr())
		expect([]rune("]"))
		node = new_unary(ND_DEREF, exp)
	}
	return node
}

func primary() *Node {
	tok := token
	
	if consume([]rune("(")) != nil {
		if consume([]rune("{")) != nil {
			return stmt_expr(tok)
		}

		node := expr()
		expect([]rune(")"))
		return node
	}

	if tok := consume([]rune("sizeof")); tok != nil {
		return new_unary(ND_SIZEOF, unary())
	}

	if tok := consume_ident(); tok != nil {
		if consume([]rune("(")) != nil {
			args := func_args()
			return &Node{kind: ND_FUNCALL, funcname: tok.str[:tok.len], args: args}
		}

		va := find_var(tok)
		if va == nil {
			fmt.Println("undefined variable")
		}
		return new_var(va)
	}
	
	if tok.kind == TK_STR {
		token = token.next
		ty := array_of(char_type(), tok.cont_len)
		va := push_var(new_label(), ty, false)
		va.contents = tok.contents
		va.cont_len = tok.cont_len
		return new_var(va)
	}

	return new_number(expect_number())
}

