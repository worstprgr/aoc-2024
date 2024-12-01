.PHONY: all build

all: build exec

build:
	nasm -g -f elf64 prog.asm -o prog.o
	ld prog.o -o prog

exec:
	./prog
