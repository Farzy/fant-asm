# Makefile

bins := $(patsubst %.asm,%,$(wildcard *_macos.asm))

LDOPTS=-macosx_version_min 10.7.0

.PHONY : all clean

all: $(bins)

%_macos: %_macos.o
	ld $(LDOPTS) -o $@ $<

%_macos.o: %_macos.asm
	nasm -fmacho64 $<

clean:
	rm -f *.o $(bins)
