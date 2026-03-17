#include <iostream>
#include <memory>
#include <vector>
#include <cstring>

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
	int len;

	Token() = default;
};

Token *token;

Token *tokenize(char *p);
Token *new_token(TokenKind kind, Token *cur, char *str, int len);

Token *tokenize(char *p) {
	Token head;
	head.next = NULL;
	Token *cur = &head;

	while (*p) {
		if (isspace(*p)) {
			p++;
			continue;
		}

		if (*p == '=' && *(p+1) == '=') {
			cur = new_token(TK_RESERVED, cur, p, 2);
			p += 2;
			continue;
		}

		if (*p == '!' && *(p+1) == '=') {
			cur = new_token(TK_RESERVED, cur, p, 2);
			p += 2;
			continue;
		}

		if (*p == '<') {
			if (*(p+1) == '=') {
				cur = new_token(TK_RESERVED, cur, p, 2);
				p += 2;
				continue;
			} else {
				cur = new_token(TK_RESERVED, cur, p, 1);
				p += 1;
				continue;
			}
		}

		if (*p == '>') {
			if (*(p+1) == '=') {
				cur = new_token(TK_RESERVED, cur, p, 2);
				p += 2;
				continue;
			} else {
				cur = new_token(TK_RESERVED, cur, p, 1);
				p += 1;
				continue;
			}
		}

		if (*p == '+' || *p == '-' || *p == '*' || *p == '/' || *p =='(' || *p == ')') {
			cur = new_token(TK_RESERVED, cur, p, 1);
			p += 1;
			continue;
		}

		if (isdigit(*p)) {
			cur = new_token(TK_NUM, cur, p, 0);
			char *q = p;
			cur->val = strtol(p, &p, 10);
			cur->len = p - q;
			continue;
		}
		std::cerr << "invalid a number\n";
	}

	new_token(TK_EOF, cur, p, 0);
	return head.next;
}

Token *new_token(TokenKind kind, Token *cur, char *str, int len) {
	Token *tok = new Token();
	tok->kind = kind;
	tok->str = str;
	tok->len = len;
	cur->next = tok;
	return tok;
}

enum NodeKind {
	ND_ADD,
	ND_SUB,
	ND_MUL,
	ND_DIV,
	ND_EQ,
	ND_NE,
	ND_LT,
	ND_LE,
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
Node *equality();
Node *relational();
Node *add();
Node *mul();
Node *unary();
Node *primary();

bool consume(char *op);
void expect(char *op);
int expect_number();
Node *new_num(int val);


bool consume(const char *op) {
	if (token->kind != TK_RESERVED || strlen(op) != token->len || memcmp(token->str, op, token->len))
		return false;
	token = token->next;
	return true;
}

void expect(const char *op) {
	if (token->kind != TK_RESERVED || strlen(op) != token->len || memcmp(token->str, op, token->len))
		std::cerr  << "expect " << op << "\n";
	token = token->next;
}

int expect_number() {
	if (token->kind != TK_NUM) {
		std::cerr << "excepted a number\n";
		exit(1);
	}
	int val = token->val;
	token = token->next;
	return val;
}

Node *new_num(int val) {
	return new Node(val);
}




Node *expr() {
	return equality();
}

Node *equality() {
	Node *node = relational();

	for (;;) {
		if (consume("==")) 
			node = new Node(ND_EQ, node, relational());
		else if (consume("!=")) 
			node = new Node(ND_NE, node, relational());
		else
			return node;
	}
}

Node *relational() {
	Node *node = add();

	for (;;) {
		if (consume("<"))
			node = new Node(ND_LT, node, add());
		else if (consume("<="))
			node = new Node(ND_LE, node, add());
		else if (consume(">"))
			node = new Node(ND_LT, add(), node);
		else if (consume(">="))
			node = new Node(ND_LE, add(), node);
		else
			return node;
	}
}

Node *add() {
	Node *node = mul();

	for (;;) {
		if (consume("+"))
			node = new Node(ND_ADD, node, mul());
		else if (consume("-"))
			node = new Node(ND_SUB, node, mul());
		else
			return node;
	}
}

Node *mul() {
	Node *node = unary();

	for (;;) {
		if (consume("*"))
			node = new Node(ND_MUL, node, unary());
		else if (consume("/"))
			node = new Node(ND_DIV, node, unary());
		else
			return node;
	}
}

Node *unary() {
	if (consume("+"))
		return unary();
	if (consume("-"))
		return new Node(ND_SUB, new_num(0), unary());
	return primary();
}

Node *primary() {
	if (consume("(")) {
		Node *node = expr();
		expect(")");
		return node;
	}
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

