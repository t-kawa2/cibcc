package main

import (
	"fmt"
)

var labelseq = 0
var argreg1 = [6]string{"dil", "sil", "dl", "cl", "r8b", "r9b"}
var argreg8 = [6]string{"rdi", "rsi", "rdx", "rcx", "r8", "r9"}
var funcname []rune

func gen(node *Node) {
	if node.kind == ND_NULL {
		return
	}
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
		if node.ty.kind != TY_ARRAY {
			load(node.ty)
		}
		return
	}
	if node.kind == ND_ASSIGN {
		gen_lval(node.lhs)
		gen(node.rhs)
		store(node.ty)
		return
	}
	if node.kind == ND_ADDR {
		gen_addr(node.lhs)
		return
	}
	if node.kind == ND_DEREF {
		gen(node.lhs)
		if node.ty.kind != TY_ARRAY {
			load(node.ty)
		}
		return
	}
	if node.kind == ND_IF {
		seq := labelseq
		labelseq++
		if node.els != nil {
			gen(node.cond)
			fmt.Println("  pop rax")
			fmt.Println("  cmp rax, 0")
			fmt.Printf("  je .Lelse%d\n", seq)
			gen(node.then)
			fmt.Printf("  jmp .Lend%d\n", seq)
			fmt.Printf(".Lelse%d:\n", seq)
			gen(node.els)
			fmt.Printf(".Lend%d:\n", seq)
		} else {
			gen(node.cond)
			fmt.Println("  pop rax")
			fmt.Println("  cmp rax, 0")
			fmt.Printf("  je .Lend%d\n", seq)
			gen(node.then)
			fmt.Printf(".Lend%d:\n", seq)
		}
		return
	}
	if node.kind == ND_WHILE {
		seq := labelseq
		labelseq++
		fmt.Printf(".Lbegin%d:\n", seq)
		gen(node.cond)
		fmt.Println("  pop rax")
		fmt.Println("  cmp rax, 0")
		fmt.Printf("  je .Lend%d\n", seq)
		gen(node.then)
		fmt.Printf("  jmp .Lbegin%d\n", seq)
		fmt.Printf(".Lend%d:\n", seq)
		return
	}
	if node.kind == ND_FOR {
		seq := labelseq
		labelseq++
		if node.init != nil {
			gen(node.init)
		}
		fmt.Printf(".Lbegin%d:\n", seq)
		if node.cond != nil {
			gen(node.cond)
			fmt.Println("  pop rax")
			fmt.Println("  cmp rax, 0")
			fmt.Printf("  je .Lend%d\n", seq)
		}
		gen(node.then)
		if node.inc != nil {
			gen(node.inc)
		}
		fmt.Printf("  jmp .Lbegin%d\n", seq)
		fmt.Printf(".Lend%d:\n", seq)
		return
	}
	if node.kind == ND_BLOCK {
		for n := node.body; n != nil; n = n.next {
			gen(n)
		}
		return
	}
	if node.kind == ND_FUNCALL {
		nargs := 0
		for n := node.args; n != nil; n = n.next {
			gen(n)
			nargs++
		}
		for i := nargs-1; i >= 0;i-- {
			fmt.Printf("  pop %s\n", argreg8[i])
		}
		seq := labelseq
		labelseq++
		fmt.Printf("  mov rax, rsp\n")
		fmt.Printf("  and rax, 15\n")
		fmt.Printf("  jnz .Lcall%d\n", seq)
		fmt.Printf("  mov rax, 0\n")
		fmt.Printf("  call %s\n", string(node.funcname))
		fmt.Printf("  jmp .Lend%d\n", seq)
		fmt.Printf(".Lcall%d:\n", seq)
		fmt.Printf("  sub rsp, 8\n")
		fmt.Printf("  mov rax, 0\n")
		fmt.Printf("  call %s\n", string(node.funcname))
		fmt.Printf("  add rsp, 8\n")
		fmt.Printf(".Lend%d:\n", seq)
		fmt.Printf("  push rax\n");
		return
	}
	if node.kind == ND_RETURN {
		gen(node.lhs)
		fmt.Println("  pop rax")
		fmt.Printf("  jmp .Lreturn.%s\n", string(funcname))
		return
	}

	gen(node.lhs)
	gen(node.rhs)

	fmt.Println("  pop rdi")
	fmt.Println("  pop rax")

	switch node.kind {
		case ND_ADD:
			if node.ty.base != nil {
				fmt.Printf("  imul rdi, %d\n", size_of(node.ty.base))
			}
			fmt.Println("  add rax, rdi")
		case ND_SUB:
			if node.ty.base != nil {
				fmt.Printf("  imul rdi, %d\n", size_of(node.ty.base))
			}
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

func codegen(prog *Program) {
	fmt.Println(".intel_syntax noprefix")
	emit_data(prog)
	emit_text(prog)
}

func emit_data(prog *Program) {
	fmt.Println(".data")

	for vl:= prog.globals; vl != nil; vl = vl.next {
		v := vl.va
		fmt.Printf("%s:\n", string(v.name))

		if v.contents != nil {
			for _, b := range v.contents {
				fmt.Printf("  .byte %d\n", b)
			}
		} else {
			fmt.Printf("  .zero %d\n", size_of(v.ty))
		}
	}
}

func emit_text(prog *Program) {
	fmt.Println(".text")

	for fn := prog.fns; fn != nil; fn = fn.next {
		fmt.Printf(".global %s\n", string(fn.name))
		fmt.Printf("%s:\n", string(fn.name))
		funcname = fn.name

		fmt.Println("  push rbp")
		fmt.Println("  mov rbp, rsp")
		fmt.Printf("  sub rsp, %d\n", fn.stack_size)

		i := 0
		for vl := fn.params; vl != nil; vl = vl.next {
			load_arg(vl.va, i)
			i++
		}

		for n := fn.node; n != nil; n = n.next {
			gen(n)
		}

		fmt.Printf(".Lreturn.%s:\n", string(funcname))
		fmt.Println("  mov rsp, rbp")
		fmt.Println("  pop rbp")
		fmt.Println("  ret")
	}
}

func gen_addr(node *Node) {
	if node.kind == ND_VAR {
		if node.va.islocal {
			fmt.Printf("  lea rax, [rbp-%d]\n", node.va.offset)
			fmt.Println("  push rax")
		} else {
			fmt.Printf("  push offset %s\n", string(node.va.name))
		}
		return
	}
	if node.kind == ND_DEREF {
		gen(node.lhs)
		return
	}
	fmt.Println("not an value")
}

func load(ty *Type) {
	fmt.Println("  pop rax")
	if size_of(ty) == 1 {
		fmt.Println("  movsx rax, byte ptr [rax]")
	} else {
		fmt.Println("  mov rax, [rax]")
	}
	fmt.Println("  push rax")
}

func store(ty *Type) {
	fmt.Println("  pop rdi")
	fmt.Println("  pop rax")
	if size_of(ty) == 1 {
		fmt.Println("  mov [rax], dil")
	} else {
		fmt.Println("  mov [rax], rdi")
	}
	fmt.Println("  push rdi")
}

func gen_lval(node *Node) {
	if node.ty.kind == TY_ARRAY {
		fmt.Println("not an lvalue")
	}
	gen_addr(node)
}

func load_arg(va *Var, idx int) {
	sz := size_of(va.ty)
	if sz == 1 {
		fmt.Printf("  mov [rbp-%d], %s\n", va.offset, argreg1[idx])
	} else {
		fmt.Printf("  mov [rbp-%d], %s\n", va.offset, argreg8[idx])
	}
}

