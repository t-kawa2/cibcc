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
	ND_RETURN
	ND_EXPR_STMT
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


func program() *Node {
	var head Node
	head.next = nil
	cur := &head

	for !at_eof() {
		cur.next = stmt()
		cur = cur.next
	}
	return head.next
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

