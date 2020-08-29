# Fant-asm

Learning x86 assembly and making stupid puns.

*Note*: Some texts have been copied from other sites, full references are provided at the
end of this document.

# Usage

## Compiling

```shell
make all
```

## Cleaning

```shell
make clean
```

# Learnings

## 32 bits vs 64 bits

Most of the ASM samples I found so far were written for 32 bits systems. I replaced the registers with
the 64 bits versions and it worked so far…
For example `eax` becomes `rax`.

Be careful not to clobber memory by using a too big register when a 16 or 32 bits write is enough!


## Linux vs macOS

* The system call numbers are different!
* For `write` and `exit` at least, the same arguments are passed in the same registers

The reason why they use the same register is because both OS on 64 bits architectures
adopted the [System V AMD64 ABI reference](https://gitlab.com/x86-psABIs/x86-64-ABI)
calling convention.

## macOS version for the linker

Linking for **macOS min version 10.7** needs the entry point to be named `start` and no dynamic library linking.

In order to compile & link for **macOS min version 10.12** you need to:
* Remplace the symbol `start` with `_main`
* Link to the `System` dynamic library with `-lSystem`

## Finding the XNU version corresponding to an OS X version

* Go to https://opensource.apple.com/
* Find your OS X version
* Find the `xnu` directory and note the version

For example, for **[macOS 10.15.6](https://opensource.apple.com/release/macos-10156.html)** 
we have **[xnu-6153.141.1](https://opensource.apple.com/source/xnu/xnu-6153.141.1/)**.

## Finding system call tables

* Find the XNU version
* Navigate to the **xnu** source code
* Find the file `bsd/kern/syscalls.master`

For example: https://opensource.apple.com/source/xnu/xnu-6153.141.1/bsd/kern/syscalls.master

## System calls on X86-64

* Arguments are passed on the registers `rdi`, `rsi`, `rdx`, `r10`, `r8` and `r9`
* Syscall number is in the `rax` register
* The call is done via the `syscall` instruction

## Relative versus absolute addressing

Most sample codes that I found use absolute memory addressing, apparently because it's how
it works on 32 bits systems.

64 bits macOS uses relative memory addressing by default in order to activate
 **[PIC ("position-independent code")](https://en.wikipedia.org/wiki/Position-independent_code)**, also
called **PIE** ("position-independent executable"). 

If the compiled `.o` file contains absolute adresssing, `ld` complains with the following
message but compiles anyway:

> ld: warning: PIE disabled. Absolute addressing (perhaps -mdynamic-no-pic) not allowed in code signed PIE, but used in _main 
> from hello_macos.o. To fix this warning, don't compile with -mdynamic-no-pic or link with -Wl,-no_pie

In order to convert to PIE style addressing, add this line at the top of the source code:

```nasm
        default   rel
```

Then convert these calls:

```nasm
        mov       rsi, message            ; address of string to output
```

to:

```nasm
        lea       rsi, [message]      ; address of string to output
```


# Licence

Copyright 2020 Farzad FARID <farzy@farzy.org>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
   
# References

## Starters

* [NASM tutorial](https://cs.lmu.edu/~ray/notes/nasmtutorial/)
* [‘Hello World’ Assembly Program on macOS Mojave](https://medium.com/@thisura1998/hello-world-assembly-program-on-macos-mojave-d5d65f0ce7c6)
* [Making system calls from Assembly in Mac OS X](https://filippo.io/making-system-calls-from-assembly-in-mac-os-x/)
* System call conventions on macOS:
  * https://stackoverflow.com/questions/11179400/basic-assembly-not-working-on-mac-x86-64lion
  * https://stackoverflow.com/questions/48845697/macos-64-bit-system-call-table
* [32-bit absolute addresses no longer allowed in x86-64 Linux?](https://stackoverflow.com/questions/43367427/32-bit-absolute-addresses-no-longer-allowed-in-x86-64-linux)

## Full courses

* [x86-64 Assembly Language Programming with Ubuntu](http://www.egr.unlv.edu/~ed/assembly64.pdf)
* [Assembly Language Tutorials and Courses on Hackr.io](https://hackr.io/tutorials/learn-assembly-language)

## Reference manuals

* [Intel® 64 and IA-32 Architectures Software Developer’s Manual Combined Volumes: 1, 2A, 2B, 2C, 2D, 3A, 3B, 3C, 3D, and 4](https://software.intel.com/content/www/us/en/develop/download/intel-64-and-ia-32-architectures-sdm-combined-volumes-1-2a-2b-2c-2d-3a-3b-3c-3d-and-4.html): 5038 pages! Updated in October 2019.
* [System V AMD64 ABI reference](https://gitlab.com/x86-psABIs/x86-64-ABI)
