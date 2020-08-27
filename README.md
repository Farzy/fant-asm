# Fant-asm

Learning x86 assembly and making stupid puns.

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
* For `write` and `exit` at least, the arguments are passed in the same registers


## macOS version

Linking for **macOS min version 10.7** needs the entry point to be named `start` and no dynamic library linking.

In order to compile & link for **macOS min version 10.12** you need to:
* Remplace the symbol `start` with `_main`
* Link to the `System` dynamic library with `-lSystem`


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

* NASM tutorial: https://cs.lmu.edu/~ray/notes/nasmtutorial/
* ‘Hello World’ Assembly Program on macOS Mojave: https://medium.com/@thisura1998/hello-world-assembly-program-on-macos-mojave-d5d65f0ce7c6

