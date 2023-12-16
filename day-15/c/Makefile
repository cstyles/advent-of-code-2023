a.out: 15.c
	clang 15.c
	# clang -O3 15.c

build: a.out

run: build
	./a.out

clean:
	rm a.out

all: build run
