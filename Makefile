# Makefile

bins := $(patsubst %.asm,bin/%,$(wildcard *_macos.asm))

LDOPTS=-macosx_version_min 10.12 -lSystem

.PHONY : all clean

all: $(bins)

bin/%_macos: %_macos.o
	ld $(LDOPTS) -o $@ $<

%_macos.o: %_macos.asm
	nasm -fmacho64 $<

clean:
	rm -f *.o $(bins)
