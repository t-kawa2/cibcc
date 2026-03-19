CC = gcc
OBJFILES = lex.yy.o y.tab.o func.o code1.o

int1 :	${OBJFILES}
	${CC} ${OBJFILES} -ll -o int1

func.o :	func.c func.h

code1.o :	code1.c func.h

lex.yy.o :	lex.yy.c y.tab.h func.h

lex.yy.c :	int1.l
	flex -l int1.l

y.tab.o :	y.tab.c func.h

y.tab.c y.tab.h :	int1.y
	bison -dv -y int1.y

clean:
	rm -f *.o lex.* y.* int1
