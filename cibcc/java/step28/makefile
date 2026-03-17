JAVACMD=javac
JAVABUILD=$(JAVACMD) Cib.java Tokenize.java Parser.java Codegen.java Type.java LabelGenerator.java

build:
	$(JAVABUILD)

test:
	java Cib tests > tmp.s
	gcc -static -o tmp tmp.s
	./tmp
	@echo "All test passed"

clean:
	rm -f *.o tmp* *.class

.PHONY:	test clean

