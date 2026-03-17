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
	ND_EXPR_STMT,
	ND_LVAR,
	ND_NUM,
};

struct Node {
	NodeKind kind;
	Node *next;
	Node *lhs;
	Node *rhs;
	char name;
	int val;

	Node() = default;

	Node(int v) : kind(ND_NUM), val(v) {}

	Node(NodeKind k, Node *l, Node *r) : kind(k), lhs(l), rhs(r) {}
};

Node *program();

		
void codegen(Node *node);

