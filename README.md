### tktmalb_rpie

Compile by specifying the linker script

``` bash
λ cargo rustc -- -C link-args=--script=./linker.ld
```

``` asm
Disassembly of section .text._start:

00008010 <_start>:
    8010:       eaffffff        b       8014 <_start+0x4>
    8014:       eafffffe        b       8014 <_start+0x4>
```