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
	char *name;
	int offset;

	Var() = default;
};

struct VarList {
	VarList *next;
	Var *var;

	VarList() = default;
};

enum TypeKind {
	TY_INT,
	TY_PTR,
};

struct Type {
	TypeKind kind;
	Type *base;

	Type() = default;
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
	ND_ADDR,
	ND_DEREF,
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
	Type *ty;
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

struct Function {
	Function *next;
	char *name;
	VarList *params;
	Node *node;
	VarList *locals;
	int stack_size;

	Function() = default;
};

Function *program();

		
void codegen(Function *prog);


void add_type(Function *prog);
