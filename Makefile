# Makefile

BIN=hello_macos

LDOPTS=-macosx_version_min 10.7.0

#hello_macos: hello_macos.o
#	ld $(LDOPTS) -o hello_macos hello_macos.o

%: %.o
	ld $(LDOPTS) -o $@ $<

%.o: %.asm
	nasm -fmacho64 $<

all: $(BIN)

.PHONY : clean
clean:
	rm -f *.o $(BIN)
