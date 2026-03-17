#include "cibcc.h"

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

