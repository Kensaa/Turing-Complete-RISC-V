.section .text
.global _start

_start:

li  x5, -1
andi x6, x5, -4
li  x7, 0xFFFFFFFC
bne x6, x7, error

li  x5, 0xFFFFFFFF
andi x6, x5, 124
li  x7, 124
bne x6, x7, error

li x5, 0xFFFFFFFF
li x6, 1
add x7, x5, x6
li x8, 0
bne x7, x8, error

sltu x9, x7, x5   # carry?
li x10, 1
bne x9, x10, error

li x5, 0
li x6, 1
sub x7, x5, x6
li x8, -1
bne x7, x8, error

sltu x9, x5, x6   # borrow?
li x10, 1
bne x9, x10, error

li x5, 0x80000000
li x6, 0
sltu x7, x5, x6   # unsigned: false
li x8, 0
bne x7, x8, error

li x5, 0
li x6, 0x80000000
sltu x7, x5, x6   # unsigned: true
li x8, 1
bne x7, x8, error

# Halt (success)
halt:
    j halt

# Error handler: trigger ebreak
error:
    ebreak
