package main

import (
	"fmt"
)

func gen(node *Node) {
	if node.kind == ND_NUM {
		fmt.Printf("  push %d\n", node.val)
		return
	}
	if node.kind == ND_RETURN {
		gen(node.lhs)
		fmt.Println("  pop rax")
		fmt.Println("  ret")
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

