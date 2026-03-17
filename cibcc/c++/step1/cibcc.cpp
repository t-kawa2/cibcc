#include <iostream>

int main(int argc, char* argv[]) {
	if (argc != 2) {
		std::cout << "引数の個数が正しくありません\n";
	return 1;
	}

	std::cout << ".intel_syntax noprefix\n";
	std::cout << ".global main\n";
	std::cout << "main:\n";

	std::cout << "  mov rax, " << atoi(argv[1]) << "\n";
	std::cout << "  ret\n";
	return 0;
}


