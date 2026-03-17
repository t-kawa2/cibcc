#include "cibcc.h"
		
void gen(Node *node);
void codegen(Node *node);
void gen_addr(Node *node);
void load();
void store();

void gen(Node *node) {
	if (node->kind == ND_NUM) {
		std::cout << "  push " << node->val << "\n";
		return;
	}
	if (node->kind == ND_EXPR_STMT) {
		gen(node->lhs);
		std::cout << "  add rsp, 8\n";
		return;
	}
	if (node->kind == ND_LVAR) {
		gen_addr(node);
		load();
		return;
	}
	if (node->kind == ND_ASSIGN) {
		gen_addr(node->lhs);
		gen(node->rhs);
		store();
		return;
	}
	if (node->kind == ND_RETURN) {
		gen(node->lhs);
		std::cout << "  pop rax\n";
		std::cout << "  jmp .Lreturn\n";
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

void codegen(Node *node) {
	std::cout << ".intel_syntax noprefix\n";
	std::cout << ".global main\n";
	std::cout << "main:\n";

	std::cout << "  push rbp\n";
	std::cout << "  mov rbp, rsp\n";
	std::cout << "  sub rsp, 208\n";

	for (Node *n = node; n; n = n->next)
		gen(n);

	std::cout << ".Lreturn:\n";
	std::cout << "  mov rsp, rbp\n";
	std::cout << "  pop rbp\n";
	std::cout << "  ret\n";
}

void gen_addr(Node *node) {
	if (node->kind == ND_LVAR) {
		int offset = (node->name - 'a' + 1) * 8;
		std::cout << "  lea rax, [rbp-" << offset << "]\n";
		std::cout << "  push rax\n";
		return;
	}

	std::cerr << "not an lvalue\n";
}

void load() {
	std::cout << "  pop rax\n";
	std::cout << "  mov rax, [rax]\n";
	std::cout << "  push rax\n";
}

void store() {
	std::cout << "  pop rdi\n";
	std::cout << "  pop rax\n";
	std::cout << "  mov [rax], rdi\n";
	std::cout << "  push rdi\n";
}

