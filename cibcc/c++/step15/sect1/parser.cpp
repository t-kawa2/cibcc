#include "cibcc.h"

Function *function();
Function *program();
Node *stmt();
Node *expr();
Node *assign();
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
Token *consume_ident();
char *expect_ident();
Node *new_var(Var *var);
Var *find_var(Token *tok);
Var *push_var(char *name);
Node *read_expr_stmt();
Node *func_args();

Token *token;
Var *locals;

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

Token *consume_ident() {
	if (token->kind != TK_IDENT)
		return NULL;
	Token *t = token;
	token = token->next;
	return t;
}

char *expect_ident() {
	if (token->kind != TK_IDENT)
		std::cerr << "expected an identifier:" << token->str << "\n";
	char *s = strndup(token->str, token->len);
	token = token->next;
	return s;
}

Node *new_var(Var *var) {
	Node *node = new Node();
	node->kind = ND_VAR;
	node->var = var;
	return node;
}

Var *find_var(Token *tok) {
	for (Var *var = locals; var; var = var->next) {
		if (strlen(var->name) == tok->len && !memcmp(tok->str, var->name, tok->len)) {
			return var;
		}
	}
	return NULL;
}

Var *push_var(char *name) {
	Var *var = new Var();
	var->next = locals;
	var->name = name;
	locals = var;
	return var;
}

Node *read_expr_stmt() {
	return new_unary(ND_EXPR_STMT, expr());
}

Node *func_args() {
	if (consume(")"))
		return NULL;

	Node *head = assign();
	Node *cur = head;
	while (consume(",")) {
		cur->next = assign();
		cur = cur->next;
	}

	expect(")");
	return head;
}





Function *function() {
	locals = NULL;

	char *name = expect_ident();
	expect("(");
	expect(")");
	expect("{");

	Node head;
	head.next = NULL;
	Node *cur = &head;

	while (!consume("}")) {
		cur->next = stmt();
		cur = cur->next;
	}

	Function *fn = new Function();
	fn->name = name;
	fn->node = head.next;
	fn->locals = locals;
	return fn;
}

Function *program() {
	Function head;
	head.next = NULL;
	Function *cur = &head;

	while (!at_eof()) {
		cur->next = function();
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

	if (consume("if")) {
		Node *node = new  Node(ND_IF);
		expect("(");
		node->cond = expr();
		expect(")");
		node->then = stmt();
		if (consume("else")) 
			node->els = stmt();
		return node;
	}

	if (consume("while")) {
		Node *node = new Node(ND_WHILE);
		expect("(");
		node->cond = expr();
		expect(")");
		node->then = stmt();
		return node;
	}

	if (consume("for")) {
		Node *node = new Node(ND_FOR);
		expect("(");
		if (!consume(";")) {
			node->init = read_expr_stmt();
			expect(";");
		}
		if (!consume(";")) {
			node->cond = expr();
			expect(";");
		}
		if (!consume(")")) {
			node->inc = read_expr_stmt();
			expect(")");
		}
		node->then = stmt();
		return node;
	}

	if (consume("{")) {
		Node head;
		head.next = NULL;
		Node *cur = &head;

		while (!consume("}")) {
			cur->next = stmt();
			cur = cur->next;
		}

		Node *node = new Node(ND_BLOCK);
		node->body = head.next;
		return node;
	}
	
	Node *node = new_unary(ND_EXPR_STMT, expr());
	expect(";");
	return node;
}

Node *expr() {
	return assign();
}

Node *assign() {
	Node *node = equality();
	if (consume("="))
		node = new Node(ND_ASSIGN, node, assign());
	return node;
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

	Token *tok = consume_ident();
	if (tok) {
		if (consume("(")) {
			Node *node = new Node(ND_FUNCALL);
			node->funcname = strndup(tok->str, tok->len);
			node->args = func_args();
			return node;
		}
		Var *var = find_var(tok);
		if (!var)
			var = push_var(strndup(tok->str, tok->len));
		return new_var(var);
	}

	return new Node(expect_number());
}

