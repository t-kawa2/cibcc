package main

import (
	"os"
	"fmt"
	"io/ioutil"
)

func main() {
	if len(os.Args) != 2 {
		fmt.Println("引数の個数が正しくありません")
	}

	filename := os.Args[1]
	s := readFile(filename)
	token = tokenize([]rune(s))
	prog := program()
	add_type(prog)

	for fn := prog.fns; fn != nil; fn = fn.next {
		offset := 0
		for vl := fn.locals; vl != nil; vl = vl.next {
			va := vl.va
			offset += size_of(va.ty)
			va.offset = offset
		}
		fn.stack_size = align_to(offset, 8)
	}

	codegen(prog)
}

func align_to(n int, align int) int {
	return (n + align - 1) & ^(align - 1)
}

func readFile(path string) string {
	d, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Println(os.Stderr, err)
		os.Exit(1)
	}
	s := string(d)
	return s
}

