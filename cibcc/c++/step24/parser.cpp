#include "cibcc.h"

Function *function();
Program *program();
Node *stmt();
Node *expr();
Node *assign();
Node *equality();
Node *relational();
Node *add();
Node *mul();
Node *unary();
Node *primary();

Token *peek(const char *s);
Token *consume(const char *s);
void expect(const char *s);
int expect_number();
Node *new_num(int val);
bool at_eof();
Node *new_unary(NodeKind kind, Node *expr);
Token *consume_ident();
char *expect_ident();
Node *new_var(Var *var);
Var *find_var(Token *tok);
Var *push_var(char *name, Type *ty, bool is_local);
Node *read_expr_stmt();
Node *func_args();
VarList *read_func_param();
Node *declaration();
Type *basetype();
Type *read_type_suffix(Type *base);
Node *postfix();
bool is_function();
void global_var();
bool is_typename();

Token *token;
VarList *locals;
VarList *globals;

Token *peek(const char *s) {
	if (token->kind != TK_RESERVED || strlen(s) != token->len || memcmp(token->str, s, token->len))
		return NULL;
	return token;
}

Token *consume(const char *s) {
	if (!peek(s))
		return NULL;
	Token *t = token;
	token = token->next;
	return t;
}

void expect(const char *s) {
	if (!peek(s))
		std::cerr  << "expect " << s << "\n";
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
	for (VarList *vl = locals; vl; vl = vl->next) {
		Var *var = vl->var;
		if (strlen(var->name) == tok->len && !memcmp(tok->str, var->name, tok->len)) {
			return var;
		}
	}

	for (VarList *vl = globals; vl; vl = vl->next) {
		Var *var = vl->var;
		if (strlen(var->name) == tok->len && !memcmp(tok->str, var->name, tok->len)) {
			return var;
		}
	}
	return NULL;
}

Var *push_var(char *name, Type *ty, bool is_local) {
	Var *var = new Var();
	var->name = name;
	var->ty = ty;
	var->is_local = is_local;

	VarList *vl = new VarList();
	vl->var = var;

	if (is_local) {
		vl->next = locals;
		locals = vl;
	} else {
		vl->next = globals;
		globals = vl;
	}

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

VarList *read_func_params() {
	if (consume(")"))
		return NULL;

	VarList *head = read_func_param();
	VarList *cur = head;

	while (!consume(")")) {
		expect(",");
		cur->next = read_func_param();
		cur = cur->next;
	}

	return head;
}

VarList *read_func_param() {
	Type *ty = basetype();
	char *name = expect_ident();
	ty = read_type_suffix(ty);

	VarList *vl = new VarList();
	vl->var = push_var(name, ty, true);
	return vl;
}

Node *declaration() {
	Type *ty = basetype();
	char *name = expect_ident();
	ty = read_type_suffix(ty);
	Var *var = push_var(name, ty, true);

	if (consume(";"))
		return new Node(ND_NULL);

	expect("=");
	Node *lhs = new_var(var);
	Node *rhs = expr();
	expect(";");
	Node *node = new Node(ND_ASSIGN, lhs, rhs);
	return new_unary(ND_EXPR_STMT, node);
}

Type *basetype() {
	Type *ty;
	if (consume("char")) {
		ty = char_type();
	} else {
		expect("int");
		ty = int_type();
	}

	while (consume("*"))
		ty = pointer_to(ty);
	return ty;
}

Type *read_type_suffix(Type *base) {
	if (!consume("["))
		return base;
	int sz = expect_number();
	expect("]");
	base = read_type_suffix(base);
	return array_of(base, sz);
}

bool is_function() {
	Token *tok = token;
	basetype();
	bool isfunc = consume_ident() && consume("(");
	token = tok;
	return isfunc;
}

void global_var() {
	Type *ty = basetype();
	char *name = expect_ident();
	ty = read_type_suffix(ty);
	expect(";");
	push_var(name, ty, false);
}

bool is_typename() {
	return peek("char") || peek("int");
}





Function *function() {
	locals = NULL;

	Function *fn = new Function();
	basetype();
	fn->name = expect_ident();
	expect("(");
	fn->params = read_func_params();
	expect("{");

	Node head;
	head.next = NULL;
	Node *cur = &head;

	while (!consume("}")) {
		cur->next = stmt();
		cur = cur->next;
	}

	fn->node = head.next;
	fn->locals = locals;
	return fn;
}

Program *program() {
	Function head;
	head.next = NULL;
	Function *cur = &head;
	globals = NULL;

	while (!at_eof()) {
		if (is_function()) {
			cur->next = function();
			cur = cur->next;
		} else {
			global_var();
		}
	}

	Program *prog = new Program();
	prog->globals = globals;
	prog->fns = head.next;
	return prog;
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

	if (is_typename())
		return declaration();
	
	Node *node = read_expr_stmt();
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
	if (consume("&"))
		return new_unary(ND_ADDR, unary());
	if (consume("*"))
		return new_unary(ND_DEREF, unary());
	return postfix();
}

Node *postfix() {
	Node *node = primary();

	while (consume("[")) {
		Node *exp = new Node(ND_ADD, node, expr());
		expect("]");
		node = new_unary(ND_DEREF, exp);
	}
	return node;
}

Node *primary() {
	if (consume("(")) {
		Node *node = expr();
		expect(")");
		return node;
	}

	Token *tok;
	if (tok = consume("sizeof"))
		return new_unary(ND_SIZEOF, unary());

	if (tok = consume_ident()) {
		if (consume("(")) {
			Node *node = new Node(ND_FUNCALL);
			node->funcname = strndup(tok->str, tok->len);
			node->args = func_args();
			return node;
		}
		Var *var = find_var(tok);
		if (!var)
			std::cerr << "undefined variable" << std::endl;
		return new_var(var);
	}

	tok = token;
	if (tok->kind != TK_NUM)
		std::cerr << "expected expression\n";
	return new_num(expect_number());
}

