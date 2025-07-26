# RISC-V (RV32IM) CPU in Turing Complete

## Running
This is a risc-V CPU made in the game Turing Complete
The save data is in the `saves` directory
the Makefile can be used to build the rust program in `src/main.rs`, the file to import in-game is `target/riscv32im-unknown-none-elf/release/program.bin`

# Tests
There is also a small test suite in the `tests` directory, that can be built using its own Makefile. It builds the tests into `tests/build`
To know if a test passes, If the CPU reaches a `ebreak` instruction (and then halts) it means that a test failed, if it infinitely loops, it means that all tests passed

# Note
Some instructions are not fully implemented:
- All the instructions in the `SYSTEM` opcode cause the CPU to halt (because I was too lazy to implement those)
- All the instructions in the `FENCE` opcode are no-op because my implementation does not do any speculative execution, out-of-order execution, or instruction caching