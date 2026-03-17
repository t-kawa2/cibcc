#include "cibcc.h"

int main(int argc, char* argv[]) {
	if (argc != 2) {
		std::cout << "引数の個数が正しくありません\n";
	return 1;
	}

	token = tokenize(argv[1]);
	Function *prog = program();
	add_type(prog);

	for (Function *fn = prog; fn; fn = fn->next) {

		int offset = 0;
		for (VarList *vl = fn->locals; vl; vl = vl->next) {
			Var *var = vl->var;
			offset += size_of(var->ty);
			var->offset = offset;
		}
		fn->stack_size = offset;
	}
	codegen(prog);

	return 0;
}

