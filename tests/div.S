.section .text
.global _start

_start:

### ------- DIV (signed) ---------

    li x5, 10
    li x6, 2
    div x7, x5, x6      # x7 = 5
    li x20, 5
    bne x7, x20, error

    li x5, -10
    li x6, 2
    div x7, x5, x6      # x7 = -5
    li x20, -5
    bne x7, x20, error

    li x5, 10
    li x6, -2
    div x7, x5, x6      # x7 = -5
    li x20, -5
    bne x7, x20, error

    li x5, -10
    li x6, -2
    div x7, x5, x6      # x7 = 5
    li x20, 5
    bne x7, x20, error

    li x5, 1
    li x6, 0
    div x7, x5, x6      # div by 0 → -1
    li x20, -1
    bne x7, x20, error

    li x5, -2147483648  # INT_MIN
    li x6, -1
    div x7, x5, x6      # overflow → INT_MIN
    li x20, -2147483648
    bne x7, x20, error

### ------- DIVU (unsigned) ---------

    li x5, 10
    li x6, 2
    divu x7, x5, x6     # x7 = 5
    li x20, 5
    bne x7, x20, error

    li x5, -10          # 0xFFFFFFF6
    li x6, 2
    divu x7, x5, x6     # large unsigned value
    li x20, 0x7FFFFFFB  # 4294967286 / 2 = 2147483643
    bne x7, x20, error

    li x5, 1
    li x6, 0
    divu x7, x5, x6     # div by 0 → 0xFFFFFFFF
    li x20, -1
    bne x7, x20, error

halt:
    j halt

### ------- Error handler ---------
error:
    ebreak
