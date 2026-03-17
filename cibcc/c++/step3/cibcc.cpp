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

	Token() : kind(TK_EOF), next(nullptr), val(0), str(nullptr) {}

	Token(TokenKind t, char *s)
		: kind(t), next(nullptr), val(0), str(s) {}
};

Token *token;

Token *tokenize(char *p);

Token *tokenize(char *p) {
	Token head;
	Token *cur = &head;

	while (*p) {
		if (isspace(*p)) {
			p++;
			continue;
		}

		if (*p == '+' || *p == '-') {
			cur->next = new Token(TK_RESERVED, p);
			cur = cur->next;
			p++;
			continue;
		}

		if (isdigit(*p)) {
			cur->next = new Token(TK_NUM, p);
			cur = cur->next;
			cur->val = strtol(p, &p, 10);
			continue;
		}
		std::cerr << "invalid token\n";
	}

	cur->next = new Token(TK_EOF, p);
	return head.next;
}

int main(int argc, char* argv[]) {
	if (argc != 2) {
		std::cout << "引数の個数が正しくありません\n";
	return 1;
	}

	token = tokenize(argv[1]);

	std::cout << ".intel_syntax noprefix\n";
	std::cout << ".global main\n";
	std::cout << "main:\n";

	if (token->kind != TK_NUM) {
		std::cout << "数ではありません\n";
		return 1;
	}

	std::cout << "  mov rax, " << token->val << "\n";
	token = token->next;

	while (token->kind != TK_EOF) {
		if (token->kind == TK_RESERVED && *token->str == '+') {
			token = token->next;
			if (token->kind != TK_NUM) {
				std::cout << "数ではありません\n";
				return 1;
			}
			std::cout << "  add rax, " << token->val << "\n";
			token = token->next;
			continue;
		}
		if (token->kind == TK_RESERVED && *token->str == '-') {
			token = token->next;
			if (token->kind != TK_NUM) {
				std::cout << "数ではありません\n";
				return 1;
			}
			std::cout << "  sub rax, " << token->val << "\n";
			token = token->next;
			continue;
		}

		std::cout << "予期しないトークンです\n";
		return 1;
	}
	std::cout << "  ret\n";
	return 0;
}

