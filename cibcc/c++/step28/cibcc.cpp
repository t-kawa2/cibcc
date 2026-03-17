#include "cibcc.h"

int align_to(int n, int align);
char *read_file(char *path);

char *user_input;

int main(int argc, char* argv[]) {
	if (argc != 2) {
		std::cout << "引数の個数が正しくありません\n";
	return 1;
	}

	user_input = read_file(argv[1]);
	token = tokenize(user_input);
	Program *prog = program();
	add_type(prog);

	for (Function *fn = prog->fns; fn; fn = fn->next) {

		int offset = 0;
		for (VarList *vl = fn->locals; vl; vl = vl->next) {
			Var *var = vl->var;
			offset += size_of(var->ty);
			var->offset = offset;
		}
		fn->stack_size = align_to(offset, 8);
	}
	codegen(prog);

	return 0;
}

int align_to(int n, int align) {
	return (n + align - 1) & ~(align - 1);
}

char *read_file(char *path) {
	FILE *fp = fopen(path, "r");
	if (!fp)
		std::cerr << "can not open " << path << std::endl;
	int filemax = 10 * 1024 * 1024;
	char *buf = (char *)malloc(filemax);
	int size = fread(buf, 1, filemax - 2, fp);
	if (!feof(fp))
		std::cerr << "file too large\n";
	if (size == 0 || buf[size - 1] != '\n')
		buf[size++] = '\n';
	buf[size] = '\0';
	return buf;
}

