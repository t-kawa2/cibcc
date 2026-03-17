public class Codegen {

	public void codegen(Node node) {

		System.out.println(".intel_syntax noprefix");
		System.out.println(".global main");
		System.out.println("main:");

		gen(node);

		System.out.println("  pop rax");
		System.out.println("  ret");
	}

	private void gen(Node node) {

		if (node.kind == NodeKind.ND_NUM) {
			System.out.printf("  push %d\n", node.val);
			return;
		}
		if (node.kind == NodeKind.ND_RETURN) {
			gen(node.lhs);
			System.out.println("  pop rax");
			System.out.println("  ret");
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
}

