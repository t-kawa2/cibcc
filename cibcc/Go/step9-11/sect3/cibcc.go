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
	node := program()

	codegen(node)
}

