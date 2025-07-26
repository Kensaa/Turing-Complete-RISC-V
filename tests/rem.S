.section .text
.global _start

_start:

### ------- REM (signed remainder) ---------

    li x5, 10
    li x6, 3
    rem x7, x5, x6      # x7 = 1
    li x20, 1
    bne x7, x20, error

    li x5, -10
    li x6, 3
    rem x7, x5, x6      # x7 = -1
    li x20, -1
    bne x7, x20, error

    li x5, 10
    li x6, -3
    rem x7, x5, x6      # x7 = 1
    li x20, 1
    bne x7, x20, error

    li x5, -10
    li x6, -3
    rem x7, x5, x6      # x7 = -1
    li x20, -1
    bne x7, x20, error

    li x5, 1
    li x6, 0
    rem x7, x5, x6      # rem by 0 → returns dividend (rs1)
    li x20, 1
    bne x7, x20, error

    li x5, -2147483648
    li x6, -1
    rem x7, x5, x6      # remainder = 0
    li x20, 0
    bne x7, x20, error

### ------- REMU (unsigned remainder) ---------

    li x5, 10
    li x6, 3
    remu x7, x5, x6     # x7 = 1
    li x20, 1
    bne x7, x20, error

    li x5, -10          # 0xFFFFFFF6
    li x6, 3
    remu x7, x5, x6     # 4294967286 % 3 = 0
    li x20, 0
    bne x7, x20, error

    li x5, 1
    li x6, 0
    remu x7, x5, x6     # rem by 0 → rs1
    li x20, 1
    bne x7, x20, error

### ------- All tests passed ---------

halt:
    j halt

### ------- Error handler ---------
error:
    ebreak
