TARGET = riscv32im-unknown-none-elf
OUT = target/$(TARGET)/release/turing_complete_riscv
ELF = target/$(TARGET)/release/program.elf
BIN = target/$(TARGET)/release/program.bin

TEST_OUT = test_out/

build:
	rm -rf $(OUT) $(ELF) $(BIN)
	cargo build --release
	mv $(OUT) $(ELF)
	riscv64-elf-objcopy -O binary --only-section=.text $(ELF) $(BIN)

test:
	rm -rf $(TEST_OUT)
clean: 
	rm -rf target/
all: build