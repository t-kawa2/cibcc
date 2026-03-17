#include "cibcc.h"

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

		if (*p == '+' || *p == '-' || *p == '*' || *p == '/' || *p =='(' || *p == ')' || *p == ';') {
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

