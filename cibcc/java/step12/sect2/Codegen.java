public class Codegen {

	int labelseq = 0;

	public void codegen(Program prog) {

		System.out.println(".intel_syntax noprefix");
		System.out.println(".global main");
		System.out.println("main:");

		System.out.println("  push rbp");
		System.out.println("  mov rbp, rsp");
		System.out.printf("  sub rsp, %d\n", prog.stack_size);

		for (Node n = prog.node; n != null; n = n.next) {
			gen(n);
		}

		System.out.println(".Lreturn:");
		System.out.println("  mov rsp, rbp");
		System.out.println("  pop rbp");
		System.out.println("  ret");
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
		if (node.kind == NodeKind.ND_RETURN) {
			gen(node.lhs);
			System.out.println("  pop rax");
			System.out.println("  jmp .Lreturn");
			return;
		}

		gen(node.lhs);
		gen(node.rhs);

		System.out.println("  pop rdi");
		System.out.println("  pop rax");

		switch (node.kind) {
			case ND_ADD:
				System.out.println("  add rax, rdi");
				break;
			case ND_SUB:
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

	
