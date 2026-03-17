#include <iostream>
#include <memory>
#include <vector>
#include <cstring>

enum TokenKind {
	TK_RESERVED,
	TK_IDENT,
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

extern Token *token;

Token *tokenize(char *p);


struct Var {
	Var *next;
	char *name;
	int offset;

	Var() = default;
};

enum NodeKind {
	ND_ADD,
	ND_SUB,
	ND_MUL,
	ND_DIV,
	ND_EQ,
	ND_NE,
	ND_LT,
	ND_LE,
	ND_ASSIGN,
	ND_RETURN,
	ND_IF,
	ND_WHILE,
	ND_FOR,
	ND_BLOCK,
	ND_FUNCALL,
	ND_EXPR_STMT,
	ND_VAR,
	ND_NUM,
};

struct Node {
	NodeKind kind;
	Node *next;
	Node *lhs;
	Node *rhs;
	Node *cond;
	Node *then;
	Node *els;
	Node *init;
	Node *inc;
	Node *body;
	char *funcname;
	Node *args;
	Var *var;
	int val;

	Node() = default;

	Node(int v) : kind(ND_NUM), val(v) {}

	Node(NodeKind k) : kind(k) {}

	Node(NodeKind k, Node *l, Node *r) : kind(k), lhs(l), rhs(r) {}
};

struct Program {
	Node *node;
	Var *locals;
	int stack_size;

	Program() = default;
};

Program *program();

		
void codegen(Program *prog);

