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

	for fn := prog.fns; fn != nil; fn = fn.next {
		offset := 0
		for vl := fn.locals; vl != nil; vl = vl.next {
			offset += 8
			vl.va.offset = offset
		}
		fn.stack_size = offset
	}

	codegen(prog)
}

