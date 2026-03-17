#include <iostream>
#include <memory>
#include <vector>

enum TokenKind {
	TK_RESERVED,
	TK_NUM,
	TK_EOF,
};

struct Token {
	TokenKind kind;
	Token *next;
	int val;
	char *str;

	Token() = default;
};

Token *token;

Token *tokenize(char *p);
Token *new_token(TokenKind kind, Token *cur, char *str);

Token *tokenize(char *p) {
	Token head;
	head.next = NULL;
	Token *cur = &head;

	while (*p) {
		if (isspace(*p)) {
			p++;
			continue;
		}

		if (*p == '+' || *p == '-') {
			cur = new_token(TK_RESERVED, cur, p++);
			continue;
		}

		if (isdigit(*p)) {
			cur = new_token(TK_NUM, cur, p);
			cur->val = strtol(p, &p, 10);
			continue;
		}
		std::cerr << "invalid a number\n";
	}

	new_token(TK_EOF, cur, p);
	return head.next;
}

Token *new_token(TokenKind kind, Token *cur, char *str) {
	Token *tok = new Token();
	tok->kind = kind;
	tok->str = str;
	cur->next = tok;
	return tok;
}

enum NodeKind {
	ND_ADD,
	ND_SUB,
	ND_MUL,
	ND_DIV,
	ND_NUM,
};

struct Node {
	NodeKind kind;
	Node *lhs;
	Node *rhs;
	int val;

	Node(int v) : kind(ND_NUM), val(v) {}

	Node(NodeKind k, Node *l, Node *r) : kind(k), lhs(l), rhs(r) {}
};

Node *expr();
Node *primary();

bool consume(char op);
int expect_number();


bool consume(char op) {
	if (token->kind != TK_RESERVED || token->str[0] != op)
		return false;
	token = token->next;
	return true;
}

int expect_number() {
	if (token->kind != TK_NUM)
		exit(1);
	int val = token->val;
	token = token->next;
	return val;
}



Node *expr() {
	Node *node = primary();

	for (;;) {
		if (consume('+'))
			node = new Node(ND_ADD, node, primary());
		else if (consume('-'))
			node = new Node(ND_SUB, node, primary());
		else
			return node;
	}
}

Node *primary() {
	return new Node(expect_number());
}

		
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
	}

	std::cout << "  push rax\n";
}

int main(int argc, char* argv[]) {
	if (argc != 2) {
		std::cout << "引数の個数が正しくありません\n";
	return 1;
	}

	token = tokenize(argv[1]);
	Node *node = expr();

	std::cout << ".intel_syntax noprefix\n";
	std::cout << ".global main\n";
	std::cout << "main:\n";

	gen(node);

	std::cout << "  pop rax\n";
	std::cout << "  ret\n";
	return 0;
}

