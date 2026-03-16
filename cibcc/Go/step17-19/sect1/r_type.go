package main

import (
	"fmt"
)

type TypeKind int

const (
	TY_INT	TypeKind = iota
	TY_PTR
)

type Type struct {
	kind TypeKind
	base *Type
}

func add_type(prog *Function) {
	for fn := prog; fn != nil; fn = fn.next {
		for node := fn.node; node != nil; node = node.next {
			visit(node)
		}
	}
}

func visit(node *Node) {
	if node == nil {
		return
	}

	visit(node.lhs)
	visit(node.rhs)
	visit(node.cond)
	visit(node.then)
	visit(node.els)
	visit(node.init)
	visit(node.inc)

	for n := node.body; n != nil; n = n.next {
		visit(n)
	}
	for n := node.args; n != nil; n = n.next {
		visit(n)
	}

	switch node.kind {
		case ND_MUL:	fallthrough
		case ND_DIV:	fallthrough
		case ND_EQ:		fallthrough
		case ND_NE:		fallthrough
		case ND_LT:		fallthrough
		case ND_LE:		fallthrough
		case ND_VAR:	fallthrough
		case ND_FUNCALL:fallthrough
		case ND_NUM:
			node.ty = int_type()
			return
		case ND_ADD:
			if node.rhs.ty != nil && node.rhs.ty.kind == TY_PTR {
				tmp := node.lhs
				node.lhs = node.rhs
				node.rhs = tmp
			}
			if node.lhs.ty != nil {
				node.ty = node.lhs.ty
			}
			if node.rhs.ty != nil && node.rhs.ty.kind == TY_PTR {
				fmt.Println("error: invalid operands to binary +")
			}
			return
		case ND_SUB:
			if node.rhs.ty.kind == TY_PTR {
				fmt.Println("error:SUB")
			}
			node.ty = node.lhs.ty
			return
		case ND_ASSIGN:
			node.ty = node.lhs.ty
			return
		case ND_ADDR:
			node.ty = pointer_to(node.lhs.ty)
			return
		case ND_DEREF:
			if node.lhs.ty.kind == TY_PTR {
				node.ty = node.lhs.ty.base
			} else {
				node.ty = int_type()
			}
			return
	}
}

func int_type() *Type {
	ty := &Type{kind: TY_INT}
	return ty
}

func pointer_to(base *Type) *Type {
	ty := &Type{kind: TY_PTR, base: base}
	return ty
}

