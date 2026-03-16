package main

import (
	"fmt"
)

func gen(node *Node) {
	if node.kind == ND_NUM {
		fmt.Printf("  push %d\n", node.val)
		return
	}
	if node.kind == ND_EXPR_STMT {
		gen(node.lhs)
		fmt.Println("  add rsp, 8")
		return
	}
	if node.kind == ND_VAR {
		gen_addr(node)
		load()
		return
	}
	if node.kind == ND_ASSIGN {
		gen_addr(node.lhs)
		gen(node.rhs)
		store()
		return
	}
	if node.kind == ND_RETURN {
		gen(node.lhs)
		fmt.Println("  pop rax")
		fmt.Println("  jmp .Lreturn")
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

func codegen(node *Node) {
	fmt.Println(".intel_syntax noprefix")
	fmt.Println(".global main")
	fmt.Println("main:")

	fmt.Println("  push rbp")
	fmt.Println("  mov rbp, rsp")
	fmt.Println("  sub rsp, 208")

	for n := node; n != nil; n = n.next {
		gen(n)
	}

	fmt.Println(".Lreturn:")
	fmt.Println("  mov rsp, rbp")
	fmt.Println("  pop rbp")
	fmt.Println("  ret")
}

func gen_addr(node *Node) {
	if node.kind == ND_VAR {
		offset := (int(node.name[0]) - 'a' + 1) * 8
		fmt.Printf("  lea rax, [rbp-%d]\n", offset)
		fmt.Println("  push rax")
		return
	}
	fmt.Println("not an value")
}

func load() {
	fmt.Println("  pop rax")
	fmt.Println("  mov rax, [rax]")
	fmt.Println("  push rax")
}

func store() {
	fmt.Println("  pop rdi")
	fmt.Println("  pop rax")
	fmt.Println("  mov [rax], rdi")
	fmt.Println("  push rdi")
}

