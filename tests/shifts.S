.section .text
.global _start

_start:

li x5, 1
li x6, 32
srl x7, x5, x6        # shift by 32 → shift by 0
li x20, 1
bne x7, x20, error

li x5, 1
li x6, 32
sll x7, x5, x6        # shift by 32 → shift by 0
li x20, 1
bne x7, x20, error

li x5, -1
li x6, 32
sra x7, x5, x6        # shift by 32 → shift by 0
li x20, -1
bne x7, x20, error

li x5, 0x12345678
li x6, 0x9ABCDEF0
li x7, 16

srl x8, x6, x7        # hi >> 16
sll x9, x5, 16        # lo << 16
or  x10, x8, x9

li x20, 0x56789ABC
bne x10, x20, error

# Halt (success)
halt:
    j halt

# Error handler: trigger ebreak
error:
    ebreak
