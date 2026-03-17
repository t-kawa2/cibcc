#include "cibcc.h"

int main(int argc, char* argv[]) {
	if (argc != 2) {
		std::cout << "引数の個数が正しくありません\n";
	return 1;
	}

	token = tokenize(argv[1]);
	Node *node = program();

	codegen(node);

	return 0;
}

