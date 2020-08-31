# Makefile

bins := $(patsubst %.asm,bin/%,$(wildcard *_macos.asm))

LDOPTS=-macosx_version_min 10.12 -lSystem

.PHONY : all clean FORCE

all: $(bins)

# Run an arbitrary binary by stripping "run_" from the target
run_%: all
	./bin/$(subst run_,,$@)

bin/%_macos: %_macos.o
	ld $(LDOPTS) -o $@ $<

%_macos.o: %_macos.asm
	nasm -fmacho64 $<

helper: FORCE
	cargo build --release
	cp target/release/helper bin

clean:
	rm -f *.o $(bins) bin/helper
