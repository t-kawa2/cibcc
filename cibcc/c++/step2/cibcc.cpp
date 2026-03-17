#include <iostream>

int main(int argc, char* argv[]) {
	if (argc != 2) {
		std::cout << "引数の個数が正しくありません\n";
	return 1;
	}

	char *p = argv[1];

	std::cout << ".intel_syntax noprefix\n";
	std::cout << ".global main\n";
	std::cout << "main:\n";
	std::cout << "  mov rax, " << strtol(p, &p, 10) << "\n";

	while (*p) {
		if (*p == '+') {
			p++;
			std::cout << "  add rax, " << strtol(p, &p, 10) << "\n";
			continue;
		}
		if (*p == '-') {
			p++;
			std::cout << "  sub rax, " << strtol(p, &p, 10) << "\n";
			continue;
		}
	}
	std::cout << "  ret\n";
	return 0;
}


