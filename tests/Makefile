CC =  riscv64-elf-gcc
OBJCOPY = riscv64-elf-objcopy

CFLAGS = -march=rv32im -mabi=ilp32 -nostdlib
LDFLAGS = -nostdlib -nostartfiles -Ttext=0x0

SRCDIR = .
BUILDDIR = build

SOURCES := $(wildcard $(SRCDIR)/*.S)
ELFS := $(patsubst $(SRCDIR)/%.S, $(BUILDDIR)/%.elf, $(SOURCES))
BINS := $(patsubst $(SRCDIR)/%.S, $(BUILDDIR)/%.bin, $(SOURCES))

all: $(ELFS) $(BINS)

$(BUILDDIR)/%.elf: $(SRCDIR)/%.S | $(BUILDDIR)
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $<

$(BUILDDIR)/%.bin: $(BUILDDIR)/%.elf
	$(OBJCOPY) -O binary $< $@

$(BUILDDIR):
	mkdir -p $(BUILDDIR)

clean:
	rm -rf $(BUILDDIR)

.PHONY: all clean