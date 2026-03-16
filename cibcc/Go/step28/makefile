GOCMD=go
GOBUILD=$(GOCMD) build

build:
	$(GOBUILD)

test:
		./cibcc tests > tmp.s
		gcc -static -o tmp tmp.s
		./tmp
		@echo "All test passed"

clean:
		rm -f *.o tmp.s tmp cibcc

.PHONY:	test clean

