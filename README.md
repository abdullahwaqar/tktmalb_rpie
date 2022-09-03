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

BCM2837 Datasheet: https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf
Concept [at](https://archive.fosdem.org/2017/schedule/event/programming_rpi3/attachments/slides/1475/export/events/attachments/programming_rpi3/slides/1475/bare_metal_rpi3.pdf)