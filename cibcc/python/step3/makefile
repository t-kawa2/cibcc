CXX = python3
TARGET = cibcc
SRCS = cibcc.py

$(TARGET): $(SRCS)
	$(CXX) $(SRCS)

test:
		./test.sh

clean:
		rm -f *.o tmp.s tmp

.PHONY:	test clean

