CXX = g++
TARGET = cibcc
SRCS = cibcc.cpp tokenize.cpp parser.cpp codegen.cpp r_type.cpp


$(TARGET):	$(SRCS)
		$(CXX) $(SRCS) -o $(TARGET)


test:	$(TARGET)
		./cibcc tests > tmp.s
		gcc -static -o tmp tmp.s
		./tmp
		@echo "All test passed"

clean:
		rm -f *.o tmp* cibcc

.PHONY:	test clean

