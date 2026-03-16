package main

import (
	"os"
	"fmt"
)

func main() {
	if len(os.Args) != 2 {
		fmt.Println("引数の個数が正しくありません")
	}

	token = tokenize([]rune(os.Args[1]))
	node = expr()

	fmt.Println(".intel_syntax noprefix")
	fmt.Println(".global main")
	fmt.Println("main:")

	gen(node)

	fmt.Println("  pop rax")
	fmt.Println("  ret")
}

