#include "cibcc.h"
		
void gen(Node *node);

void gen(Node *node) {
	if (node->kind == ND_NUM) {
		std::cout << "  push " << node->val << "\n";
		return;
	}

	gen(node->lhs);
	gen(node->rhs);

	std::cout << "  pop rdi\n";
	std::cout << "  pop rax\n";

	switch (node->kind) {
		case ND_ADD:
			std::cout << "  add rax, rdi\n";
			break;
		case ND_SUB:
			std::cout << "  sub rax, rdi\n";
			break;
		case ND_MUL:
			std::cout << "  imul rax, rdi\n";
			break;
		case ND_DIV:
			std::cout << "  cqo\n";
			std::cout << "  idiv rdi\n";
			break;
		case ND_EQ:
			std::cout << "  cmp rax, rdi\n";
			std::cout << "  sete al\n";
			std::cout << "  movzb rax, al\n";
			break;
		case ND_NE:
			std::cout << "  cmp rax, rdi\n";
			std::cout << "  setne al\n";
			std::cout << "  movzb rax, al\n";
			break;
		case ND_LT:
			std::cout << "  cmp rax, rdi\n";
			std::cout << "  setl al\n";
			std::cout << "  movzb rax, al\n";
			break;
		case ND_LE:
			std::cout << "  cmp rax, rdi\n";
			std::cout << "  setle al\n";
			std::cout << "  movzb rax, al\n";
			break;
	}

	std::cout << "  push rax\n";
}

