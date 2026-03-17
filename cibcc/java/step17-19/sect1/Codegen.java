public class Codegen {

	int labelseq = 0;
	String argreg[] = {"rdi", "rsi", "rdx", "rcx", "r8", "r9"};
	String funcname;

	public void codegen(Function prog) {
		System.out.println(".intel_syntax noprefix");

		for (Function fn = prog; fn != null; fn = fn.next) {
			System.out.printf(".global %s\n", fn.name);
			System.out.printf("%s:\n", fn.name);
			funcname = fn.name;

			System.out.println("  push rbp");
			System.out.println("  mov rbp, rsp");
			System.out.printf("  sub rsp, %d\n", fn.stack_size);

			int i = 0;
			for (VarList vl = fn.params; vl != null; vl = vl.next) {
				Var va = vl.va;
				System.out.printf("  mov [rbp-%d], %s\n", va.offset, argreg[i++]);
			}
			for (Node n = fn.node; n != null; n = n.next) {
				gen(n);
			}

			System.out.printf(".Lreturn.%s:\n", funcname);
			System.out.println("  mov rsp, rbp");
			System.out.println("  pop rbp");
			System.out.println("  ret");
		}
	}

	private void gen(Node node) {

		if (node.kind == NodeKind.ND_NUM) {
			System.out.printf("  push %d\n", node.val);
			return;
		}
		if (node.kind == NodeKind.ND_EXPR_STMT) {
			gen(node.lhs);
			System.out.println("  add rsp, 8");
			return;
		}
		if (node.kind == NodeKind.ND_VAR) {
			gen_addr(node);
			load();
			return;
		}
		if (node.kind == NodeKind.ND_ASSIGN) {
			gen_addr(node.lhs);
			gen(node.rhs);
			store();
			return;
		}
		if (node.kind == NodeKind.ND_ADDR) {
			gen_addr(node.lhs);
			return;
		}
		if (node.kind == NodeKind.ND_DEREF) {
			gen(node.lhs);
			load();
			return;
		}
		if (node.kind == NodeKind.ND_IF) {
			int seq = labelseq;
			labelseq++;
			if (node.els !=null) {
				gen(node.cond);
				System.out.println("  pop rax");
				System.out.println("  cmp rax, 0");
				System.out.printf("  je .Lelse%d\n", seq);
				gen(node.then);
				System.out.printf(".Lelse%d:\n", seq);
				gen(node.els);
			} else {
				gen(node.cond);
				System.out.println("  pop rax");
				System.out.println("  cmp rax, 0");
				System.out.printf("  je .Lend%d\n", seq);
				gen(node.then);
				System.out.printf(".Lend%d:\n", seq);
			}
			return;
		}
		if (node.kind == NodeKind.ND_WHILE) {
			int seq = labelseq++;
			System.out.printf(".Lbegin%d:\n", seq);
			gen(node.cond);
			System.out.println("  pop rax");
			System.out.println("  cmp rax, 0");
			System.out.printf("  je .Lend%d\n", seq);
			gen(node.then);
			System.out.printf("  jmp .Lbegin%d\n", seq);
			System.out.printf(".Lend%d:\n", seq);
			return;
		}
		if (node.kind == NodeKind.ND_FOR) {
			int seq = labelseq++;
			if (node.init != null) {
				gen(node.init);
			}
			System.out.printf(".Lbegin%d:\n", seq);
			if (node.cond != null) {
				gen(node.cond);
				System.out.println("  pop rax");
				System.out.println("  cmp rax, 0");
				System.out.printf("  je .Lend%d\n", seq);
			}
			gen(node.then);
			if (node.inc != null) {
				gen(node.inc);
			}
			System.out.printf("  jmp .Lbegin%d\n", seq);
			System.out.printf(".Lend%d:\n", seq);
			return;
		}
		if (node.kind == NodeKind.ND_BLOCK) {
			for (Node n = node.body; n != null; n = n.next) {
				gen(n);
			}
			return;
		}
		if (node.kind == NodeKind.ND_FUNCALL) {
			int nargs = 0;
			for (Node arg = node.args; arg != null; arg = arg.next) {
				gen(arg);
				nargs++;
			}
			for (int i = nargs - 1; i >= 0; i--) {
				System.out.printf("  pop %s\n", argreg[i]);
			}
			int seq = labelseq++;
			System.out.println("  mov rax, rsp");
			System.out.println("  and rax, 15");
			System.out.printf("  jnz .Lcall%d\n", seq);
			System.out.println("  mov rax, 0");
			System.out.printf("  call %s\n", node.funcname);
			System.out.printf("  jmp .Lend%d\n", seq);
			System.out.printf(".Lcall%d:\n", seq);
			System.out.println("  sub rsp, 8");
			System.out.println("  mov rax, 0");
			System.out.printf("  call %s\n", node.funcname);
			System.out.println("  add rsp, 8");
			System.out.printf(".Lend%d:\n", seq);
			System.out.println("  push rax");
			return;
		}
		if (node.kind == NodeKind.ND_RETURN) {
			gen(node.lhs);
			System.out.println("  pop rax");
			System.out.printf("  jmp .Lreturn.%s\n", funcname);
			return;
		}

		gen(node.lhs);
		gen(node.rhs);

		System.out.println("  pop rdi");
		System.out.println("  pop rax");

		switch (node.kind) {
			case ND_ADD:
				if (node.ty.kind.equals(Type.TypeKind.TY_PTR)) {
					System.out.println("  imul rdi, 8");
				} else if (node.ty == null) {
					System.err.println("Error in Codegen: Node has no Type info. NodeKind: " + node.kind);
				}
				System.out.println("  add rax, rdi");
				break;
			case ND_SUB:
				if (node.ty.kind.equals(Type.TypeKind.TY_PTR)) {
					System.out.println("  imul rdi, 8");
				}
				System.out.println("  sub rax, rdi");
				break;
			case ND_MUL:
				System.out.println("  imul rax, rdi");
				break;
			case ND_DIV:
				System.out.println("  cqo");
				System.out.println("  idiv rdi");
				break;
			case ND_EQ:
				System.out.println("  cmp rax, rdi");
				System.out.println("  sete al");
				System.out.println("  movzb rax, al");
				break;
			case ND_NE:
				System.out.println("  cmp rax, rdi");
				System.out.println("  setne al");
				System.out.println("  movzb rax, al");
				break;
			case ND_LT:
				System.out.println("  cmp rax, rdi");
				System.out.println("  setl al");
				System.out.println("  movzb rax, al");
				break;
			case ND_LE:
				System.out.println("  cmp rax, rdi");
				System.out.println("  setle al");
				System.out.println("  movzb rax, al");
				break;
		}
		System.out.println("  push rax");
	}

	private void gen_addr(Node node) {
		if (node.kind == NodeKind.ND_VAR) {
			System.out.printf("  lea rax, [rbp-%d]\n", node.va.offset);
			System.out.println("  push rax");
			return;
		}
		if (node.kind == NodeKind.ND_DEREF) {
			gen(node.lhs);
			return;
		}
		System.out.println("not an lvalue");
	}

	private void load() {
		System.out.println("  pop rax");
		System.out.println("  mov rax, [rax]");
		System.out.println("  push rax");
	}

	private void store() {
		System.out.println("  pop rdi");
		System.out.println("  pop rax");
		System.out.println("  mov [rax], rdi");
		System.out.println("  push rdi");
	}
}

	
