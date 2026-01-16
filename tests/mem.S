.section .text
.global _start

_start:
li sp, 0x4000

# Allocate a 32-byte stack frame
addi sp, sp, -32
li t0, 0x12345678
sw t0, 0(sp)           # first word
li t1, 0x87654321
sw t1, 28(sp)          # last word
lw t2, 0(sp)
lw t3, 28(sp)
bne t2, t0, error
bne t3, t1, error

li sp, 0x4000         # initialize stack pointer (top of RAM)

# Check alignment of SP
andi t0, sp, 15       # SP % 16
bnez t0, error        # fail if not aligned

call test_func
j next1

# Simple function to check alignment inside a call
test_func:
andi t1, sp, 15
bnez t1, error
ret

next1:

halt:
# Halt (success)
    j halt

# Error handler: trigger ebreak
error:
    ebreak