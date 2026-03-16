package main

import (
	"fmt"
)

type TypeKind int

const (
	TY_INT	TypeKind = iota
	TY_PTR
	TY_ARRAY
)

type Type struct {
	kind TypeKind
	base *Type
	array_size int
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
		case ND_FUNCALL:fallthrough
		case ND_NUM:
			node.ty = int_type()
			return
		case ND_VAR:
			node.ty = node.va.ty
			return
		case ND_ADD:
			if node.rhs.ty.base != nil {
				tmp := node.lhs
				node.lhs = node.rhs
				node.rhs = tmp
			}
			if node.lhs.ty.base != nil {
				node.ty = node.lhs.ty
			}
			if node.rhs.ty.base != nil {
				fmt.Println("error: invalid operands to binary +")
			}
			node.ty = node.lhs.ty
			return
		case ND_SUB:
			if node.rhs.ty.base != nil {
				fmt.Println("error:SUB")
			}
			node.ty = node.lhs.ty
			return
		case ND_ASSIGN:
			node.ty = node.lhs.ty
			return
		case ND_ADDR:
			if node.lhs.ty.kind == TY_ARRAY {
				node.ty = pointer_to(node.lhs.ty.base)
			} else {
				node.ty = pointer_to(node.lhs.ty)
			}
			return
		case ND_DEREF:
			if node.lhs.ty.base == nil {
				fmt.Println("invalid pointer dereference")
			}
			node.ty = node.lhs.ty.base
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

func array_of(base *Type, size int) *Type {
	return &Type{kind: TY_ARRAY, base: base, array_size: size}
}

func size_of(ty *Type) int {
	if ty.kind == TY_INT || ty.kind == TY_PTR {
		return 8
	}
	return size_of(ty.base) * ty.array_size
}

