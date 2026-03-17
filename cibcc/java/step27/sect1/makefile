JAVACMD=javac
JAVABUILD=$(JAVACMD) Cib.java Tokenize.java Parser.java Codegen.java Type.java LabelGenerator.java

build:
	$(JAVABUILD)

test:
	./test.sh

clean:
	rm -f *.o tmp* *.class

.PHONY:	test clean

