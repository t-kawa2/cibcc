CXX = python3
TARGET = cibcc
SRCS = cibcc.py

$(TARGET): $(SRCS)
	$(CXX) $(SRCS)

test:
		python3 cibcc.py tests > tmp.s
		gcc -static -o tmp tmp.s
		./tmp
		@echo "All test passed"

clean:
		rm -f *.o tmp.s tmp
		rm -r __pycache__

.PHONY:	test clean

