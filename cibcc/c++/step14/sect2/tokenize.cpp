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

		if (*p == '+' || *p == '-' || *p == '*' || *p == '/' || *p == '(' ||
		    *p == ')' || *p == ';' || *p == '=' || *p == '{' || *p == '}' ||
			*p == ',') {
			cur = new_token(TK_RESERVED, cur, p, 1);
			p += 1;
			continue;
		}

		if (strncmp(p, "return", 6) == 0 && !isalnum(p[6])) {
			cur =new_token(TK_RESERVED, cur, p, 6);
			p += 6;
			continue;
		}

		if (strncmp(p, "if", 2) == 0 && !isalnum(p[2])) {
			cur =new_token(TK_RESERVED, cur, p, 2);
			p += 2;
			continue;
		}

		if (strncmp(p, "else", 4) == 0 && !isalnum(p[4])) {
			cur =new_token(TK_RESERVED, cur, p, 4);
			p += 4;
			continue;
		}

		if (strncmp(p, "while", 5) == 0 && !isalnum(p[5])) {
			cur =new_token(TK_RESERVED, cur, p, 5);
			p += 5;
			continue;
		}

		if (strncmp(p, "for", 3) == 0 && !isalnum(p[3])) {
			cur =new_token(TK_RESERVED, cur, p, 3);
			p += 3;
			continue;
		}

		if (isalpha(*p) || *p == '_') {
			char *q = p;
			p++;
			while (isalnum(*p) || *p == '_') {
				p++;
			}
			cur = new_token(TK_IDENT, cur, q, p - q);
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

