#!/bin/bash
assert() {
	expected="$1"
	input="$2"

	./cibcc "$input" > tmp.s
	gcc -static -o tmp tmp.s
	./tmp
	actual="$?"

	if [ "$actual" = "$expected" ]; then
		echo "$input => $actual"
	else
		echo "$input => $expected expected, but got $actual"
		exit 1
	fi
}

assert 0 'return 0;'
assert 42 'return 42;'
assert 21 'return 5+20-4;'
assert 41 'return   12  +  34  -  5  ;'
assert 8 'return 2+2*3;'
assert 6 'return 16/2-2;'
assert 15 'return 5*(9-6);'
assert 4 'return (3+5)/2;'
assert 10 'return - -10;'
assert 10 'return - - +10;'
assert 1 'return 2 > 1;'
assert 0 'return 1 > 2;'
assert 0 'return 2 < 1;'
assert 1 'return 1 < 2;'
assert 0 'return 1 == 2;'
assert 1 'return 2 == 2;'
assert 1 'return 1 != 2;'
assert 0 'return 2 != 2;'
assert 1 'return 1 <= 2;'
assert 0 'return 2 <= 1;'
assert 0 'return 1 >= 2;'
assert 1 'return 2 >= 1;'
assert 1 'return 1; 2; 3;'
assert 2 '1; return 2; 3;'
assert 3 'a=3; return a;'
assert 8 'a=3;z=5; return a+z;'

echo OK

