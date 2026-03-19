CC = gcc
OBJFILES = lex.yy.o y.tab.o func.o code2.o

int2 :	${OBJFILES}
	${CC} ${OBJFILES} -ll -o int2

func.o :	func.c func.h

code2.o :	code2.c func.h

lex.yy.o :	lex.yy.c y.tab.h func.h

lex.yy.c :	int2.l
	flex -l int2.l

y.tab.o :	y.tab.c func.h

y.tab.c y.tab.h :	int2.y
	bison -dv -y int2.y

clean:
	rm -f *.o lex.* y.* int2
