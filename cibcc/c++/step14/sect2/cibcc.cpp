#include "cibcc.h"

int main(int argc, char* argv[]) {
	if (argc != 2) {
		std::cout << "引数の個数が正しくありません\n";
	return 1;
	}

	token = tokenize(argv[1]);
	Program *prog = program();

	int offset = 0;
	for (Var *var = prog->locals; var; var = var->next) {
		offset += 8;
		var->offset = offset;
	}
	prog->stack_size = offset;

	codegen(prog);

	return 0;
}

