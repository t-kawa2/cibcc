RUSTCMD=cargo
RUSTBUILD=$(RUSTCMD)

build:
	$(RUSTBUILD) run

test:
	./test.sh

clean:
	rm -f *.o tmp* Cargo.lock
	rm -r target

.PHONY:	test clean

