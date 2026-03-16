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
	prog := program()

	offset := 0
	for va := prog.locals; va != nil; va = va.next {
		offset += 8
		va.offset = offset
	}
	prog.stack_size = offset

	codegen(prog)
}

