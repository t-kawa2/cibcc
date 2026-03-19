CC = gcc
OBJFILES = lex.yy.o y.tab.o func.o code3.o

int3 :	${OBJFILES}
	${CC} ${OBJFILES} -ll -o int3

func.o :	func.c func.h

code3.o :	code3.c func.h

lex.yy.o :	lex.yy.c y.tab.h func.h

lex.yy.c :	int3.l
	flex -l int3.l

y.tab.o :	y.tab.c func.h

y.tab.c y.tab.h :	int3.y
	bison -dv -y int3.y

clean:
	rm -f *.o lex.* y.* int3
