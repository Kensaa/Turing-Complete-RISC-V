.section .text
.global _start

_start:

### ------- MUL (signed × signed → low 32) ---------

    li x5, -8           # x5
    li x6, 8            # x6
    mul x7, x5, x6      # x7 = -64
    li x20, -64
    bne x7, x20, error

    li x5, -2147483648  # x5 INT_MIN
    li x6, -1           # x6
    mul x8, x5, x6      # x8 = INT_MIN (overflow wraps around)
    li x20, -2147483648
    bne x8, x20, error

    li x5, 2147483647   # x5 INT_MAX
    li x6, 2            # x6
    mul x9, x5, x6      # x9 = 4294967294 → -2 (wrapped)
    li x20, -2
    bne x9, x20, error

    li x5, 0            # x5
    li x6, 123456       # x6
    mul x10, x5, x6     # x10 = 0
    li x20, 0
    bne x10, x20, error

### ------- MULH (signed × signed → high 32) ---------

    li x5, -8           # x5
    li x6, 8            # x6
    mulh x11, x5, x6    # x11 = -1 (high bits of -64)
    li x20, -1
    bne x11, x20, error

    li x5, 2147483647   # x5 INT_MAX
    li x6, 2147483647   # x6 INT_MAX
    mulh x12, x5, x6    # x12 = 0x3FFFFFFF
    li x20, 0x3FFFFFFF
    bne x12, x20, error

    li x5, -2147483648  # x5 INT_MIN
    li x6, 2            # x6
    mulh x13, x5, x6    # x13 = -1
    li x20, -1
    bne x13, x20, error

### ------- MULHSU (signed × unsigned → high 32) ---------

    li x5, -8           # x5
    li x6, 8            # x6
    mulhsu x14, x5, x6  # x14 = -1
    li x20, -1
    bne x14, x20, error

    li x5, -1           # x5
    li x6, 0xFFFFFFFF   # x6 max unsigned
    mulhsu x15, x5, x6  # x15 = 0xFFFFFFFF
    li x20, 0xFFFFFFFF
    bne x15, x20, error

    li x5, 0            # x5
    li x6, 12345        # x6
    mulhsu x16, x5, x6  # x16 = 0
    li x20, 0
    bne x16, x20, error

### ------- MULHU (unsigned × unsigned → high 32) ---------

    li x5, 0xFFFFFFFF   # x5 max unsigned
    li x6, 0xFFFFFFFF   # x6
    mulhu x17, x5, x6   # x17 = 0xFFFFFFFE
    li x20, 0xFFFFFFFE
    bne x17, x20, error

    li x5, 1            # x5
    li x6, 0xFFFFFFFF   # x6
    mulhu x18, x5, x6   # x18 = 0x200000000
    li x20, 0
    bne x18, x20, error

    li x5, 123456       # x5
    li x6, 0            # x6
    mulhu x19, x5, x6   # x19 = 0
    li x20, 0
    bne x19, x20, error

# Halt (success)
halt:
    j halt

# Error handler: trigger ebreak
error:
    ebreak
