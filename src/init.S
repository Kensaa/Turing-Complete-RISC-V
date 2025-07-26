.section .text
.globl _start

_start:
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop

    la sp, _stack_end # init stack pointer
    add s0, sp, zero   # initialize frame pointer to sp
    
    call _rust_start
    
hang:
    j hang           # loop forever