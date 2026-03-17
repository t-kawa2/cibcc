CXX = g++
TARGET = cibcc
SRCS = cibcc.cpp tokenize.cpp parser.cpp codegen.cpp r_type.cpp


$(TARGET):	$(SRCS)
		$(CXX) $(SRCS) -o $(TARGET)


test:	$(TARGET)
		./test.sh

clean:
		rm -f *.o tmp.s tmp cibcc
		

.PHONY:	test clean

