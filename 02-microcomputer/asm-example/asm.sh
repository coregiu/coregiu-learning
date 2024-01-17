#/bin/bash

nasm -f elf32 sum.asm -o sum.o
ld -m elf_i386 sum.o -o sum
./sum
