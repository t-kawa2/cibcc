CXX = g++
TARGET = cibcc
SRCS = cibcc.cpp


$(TARGET):	$(SRCS)
		$(CXX) $(SRCS)　


test:	$(TARGET)
		./test.sh

clean:
		rm -f *.o tmp.s tmp cibcc

.PHONY:	test clean

