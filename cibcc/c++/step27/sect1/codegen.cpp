#include "cibcc.h"
		
void gen(Node *node);
void codegen(Program *prog);
void gen_addr(Node *node);
void load(Type *ty);
void store(Type *ty);
void gen_lval(Node *node);
void emit_data(Program *prog);
void emit_text(Program *prog);
void load_arg(Var *var, int idx);

int labelseq = 0;
const char *argreg8[] = {"rdi", "rsi", "rdx", "rcx", "r8", "r9"};
const char *argreg1[] = {"dil", "sil", "dl", "cl", "r8b", "r9b"};
char *funcname;

void gen(Node *node) {
	if (node->kind == ND_NULL) {
		return;
	}
	if (node->kind == ND_NUM) {
		std::cout << "  push " << node->val << "\n";
		return;
	}
	if (node->kind == ND_EXPR_STMT) {
		gen(node->lhs);
		std::cout << "  add rsp, 8\n";
		return;
	}
	if (node->kind == ND_VAR) {
		gen_addr(node);
		if (node->ty->kind != TY_ARRAY)
			load(node->ty);
		return;
	}
	if (node->kind == ND_ASSIGN) {
		gen_lval(node->lhs);
		gen(node->rhs);
		store(node->ty);
		return;
	}
	if (node->kind == ND_ADDR) {
		gen_addr(node->lhs);
		return;
	}
	if (node->kind == ND_DEREF) {
		gen(node->lhs);
		if (node->ty->kind != TY_ARRAY)
			load(node->ty);
		return;
	}
	if (node->kind == ND_IF) {
		int seq = labelseq++;
		if (node->els) {
			gen(node->cond);
			std::cout << "  pop rax\n";
			std::cout << "  cmp rax, 0\n";
			std::cout << "  je  .Lelse" << seq << "\n";
			gen(node->then);
			std::cout << "  jmp .Lend" << seq << "\n";
			std::cout << ".Lelse" << seq << ":\n";
			gen(node->els);
			std::cout << ".Lend" << seq << ":\n";
		} else {
			gen(node->cond);
			std::cout << "  pop rax\n";
			std::cout << "  cmp rax, 0\n";
			std::cout << "  je  .Lend" << seq << "\n";
			gen(node->then);
			std::cout << ".Lend" << seq << ":\n";
		}
		return;
	}
	if (node->kind == ND_WHILE) {
		int seq = labelseq++;
		std::cout << ".Lbegin" << seq << ":\n";
		gen(node->cond);
		std::cout << "  pop rax\n";
		std::cout << "  cmp rax, 0\n";
		std::cout << "  je  .Lend" << seq << "\n";
		gen(node->then);
		std::cout << "  jmp .Lbegin" << seq << "\n";
		std::cout << ".Lend" << seq << ":\n";
		return;
	}
	if (node->kind == ND_FOR) {
		int seq = labelseq++;
		if (node->init)
			gen(node->init);
		std::cout << ".Lbegin" << seq << ":\n";
		if (node->cond) {
			gen(node->cond);
			std::cout << "  pop rax\n";
			std::cout << "  cmp rax, 0\n";
			std::cout << "  je .Lend" << seq << "\n";
		}
		gen(node->then);
		if (node->inc)
			gen(node->inc);
		std::cout << "  jmp .Lbegin" << seq << "\n";
		std::cout << ".Lend" << seq << ":\n";
		return;
	}
	if (node->kind == ND_BLOCK || node->kind == ND_STMT_EXPR) {
		for (Node *n = node->body; n; n = n->next)
			gen(n);
		return;
	}
	if (node->kind == ND_FUNCALL) {
		int nargs = 0;
		for (Node *arg = node->args; arg; arg = arg->next) {
			gen(arg);
			nargs++;
		}

		for (int i = nargs - 1; i >= 0; i--)
			std::cout << "  pop " << argreg8[i] << "\n";

		int seq = labelseq++;
		std::cout << "  mov rax, rsp\n";
		std::cout << "  and rax, 15\n";
		std::cout << "  jnz .Lcall" << seq << "\n";
		std::cout << "  mov rax, 0\n";
		std::cout << "  call " << node->funcname << "\n";
		std::cout << "  jmp .Lend" << seq << "\n";
		std::cout << ".Lcall" << seq << ":\n";
		std::cout << "  sub rsp, 8\n";
		std::cout << "  mov rax, 0\n";
		std::cout << "  call " << node->funcname << "\n";
		std::cout << "  add rsp, 8\n";
		std::cout << ".Lend" << seq << ":\n";
		std::cout << "  push rax\n";
		return;
	}
	if (node->kind == ND_RETURN) {
		gen(node->lhs);
		std::cout << "  pop rax\n";
		std::cout << "  jmp .Lreturn." << funcname << "\n";
		return;
	}

	gen(node->lhs);
	gen(node->rhs);

	std::cout << "  pop rdi\n";
	std::cout << "  pop rax\n";

	switch (node->kind) {
		case ND_ADD:
			if (node->ty->base)
				std::cout << "  imul rdi, " << size_of(node->ty->base) << "\n";
			std::cout << "  add rax, rdi\n";
			break;
		case ND_SUB:
			if (node->ty->base)
				std::cout << "  imul rdi, " << size_of(node->ty->base) << "\n";
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

void codegen(Program *prog) {
	std::cout << ".intel_syntax noprefix\n";
	emit_data(prog);
	emit_text(prog);
}

void emit_data(Program *prog) {
	std::cout << ".data\n";

	for (VarList *vl = prog->globals; vl; vl = vl->next) {
		Var *var = vl->var;
		std::cout << var->name << ":\n";

		if (!var->contents) {
			std::cout << "  .zero " << size_of(var->ty) << "\n";
			continue;
		}

		for (int i = 0; i < var->cont_len; i++) {
			int c = (unsigned char)var->contents[i];
			std::cout << "  .byte " << c << "\n";
		}
	}
}

void emit_text(Program *prog) {
	std::cout << ".text\n";

	for (Function *fn = prog->fns; fn; fn = fn->next) {
		std::cout << ".global " << fn->name << "\n";
		std::cout << fn->name << ":\n";
		funcname = fn->name;

		std::cout << "  push rbp\n";
		std::cout << "  mov rbp, rsp\n";
		std::cout << "  sub rsp, " << fn->stack_size << "\n";

		int i = 0;
		for (VarList *vl = fn->params; vl; vl = vl->next)
			load_arg(vl->var, i++);

		for (Node *node = fn->node; node; node = node->next)
			gen(node);

		std::cout << ".Lreturn." << funcname << ":\n";
		std::cout << "  mov rsp, rbp\n";
		std::cout << "  pop rbp\n";
		std::cout << "  ret\n";
	}
}

void gen_addr(Node *node) {
	if (node->kind == ND_VAR) {
		Var *var = node->var;
		if (var->is_local) {
			std::cout << "  lea rax, [rbp-" << var->offset << "]\n";
			std::cout << "  push rax\n";
		} else {
			std::cout << "  push offset " << var->name << "\n";
		}
		return;
	}
	if (node->kind == ND_DEREF) {
		gen(node->lhs);
		return;
	}

	std::cerr << "not an lvalue\n";
}

void load(Type *ty) {
	std::cout << "  pop rax\n";
	if (size_of(ty) == 1)
		std::cout << "  movsx rax, byte ptr [rax]\n";
	else
		std::cout << "  mov rax, [rax]\n";
	std::cout << "  push rax\n";
}

void store(Type *ty) {
	std::cout << "  pop rdi\n";
	std::cout << "  pop rax\n";
	if (size_of(ty) == 1)
		std::cout << "  mov [rax], dil\n";
	else
		std::cout << "  mov [rax], rdi\n";
	std::cout << "  push rdi\n";
}

void gen_lval(Node *node) {
	if (node->ty->kind == TY_ARRAY)
		std::cerr << "not an lvalue\n";
	gen_addr(node);
}

void load_arg(Var *var, int idx) {
	int sz = size_of(var->ty);
	if (sz == 1) {
		std::cout << "  mov [rbp-" << var->offset  << "], " << argreg1[idx] << "\n";
	} else {
		assert(sz == 8);
		std::cout << "  mov [rbp-" << var->offset  << "], " << argreg8[idx] << "\n";
	}
}
		
