package main

import (
	"os"
	"fmt"
)

func isDigit(c rune) bool {
	return '0' <= c && c <= '9'
}

func strtoi(p *[]rune) int {
	s := *p
	c :=s[0]
	s = s[1:]

	acc := 0
	for {
		k := int(c - '0')
		acc *= 10
		acc += k
		if len(s) == 0 || !isDigit(s[0]) {
			break
		}
		c = s[0]
		s = s[1:]
	}
	*p = s
	return acc
}

func main() {
	if len(os.Args) != 2 {
		fmt.Println("引数の個数が正しくありません")
	}

	p := []rune(os.Args[1])

	fmt.Println(".intel_syntax noprefix")
	fmt.Println(".global main")
	fmt.Println("main:")

	fmt.Println("  mov rax, ", strtoi(&p))

	for len(p) > 0 {
		if p[0] == '+' {
			p = p[1:]
			fmt.Printf("  add rax, %d\n", strtoi(&p))
		} else if p[0] == '-' {
			p = p[1:]
			fmt.Printf("  sub rax, %d\n", strtoi(&p))
		}
	}
	fmt.Println("  ret")
}

