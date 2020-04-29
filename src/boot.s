.option norvc
.section .boot, "ax", @progbits
.global _start
.global abort

_start:
 la sp, stacks + 1024
 j __start_hello
.bss

stacks:
 .skip 1024
