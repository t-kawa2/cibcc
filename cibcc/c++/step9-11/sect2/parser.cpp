#include "cibcc.h"

Node *program();
Node *stmt();
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
bool at_eof();
Node *new_unary(NodeKind kind, Node *expr);

Token *token;

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

bool at_eof() {
	return token->kind == TK_EOF;
}

Node *new_unary(NodeKind kind, Node *expr) {
	Node *node = new Node();
	node->kind = kind;
	node->lhs = expr;
	return node;
}




Node *program() {
	Node head;
	head.next = NULL;
	Node *cur = &head;

	while (!at_eof()) {
		cur->next = stmt();
		cur = cur->next;
	}
	return head.next;
}

Node *stmt() {
	if (consume("return")) {
		Node *node = new_unary(ND_RETURN, expr());
		expect(";");
		return node;
	}

	Node *node = expr();
	expect(";");
	return node;
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

