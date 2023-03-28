TARGET := riscv64gc-unknown-none-elf
MODE := release
BIOS := ../rustsbi-qemu/rustsbi-qemu-release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

run: strip
	qemu-system-riscv64 -machine virt -nographic -bios $(BIOS) -device loader,file=$(KERNEL_BIN),addr=0x80200000

debug: strip
	qemu-system-riscv64 -machine virt -nographic -bios $(BIOS) -device loader,file=$(KERNEL_BIN),addr=0x80200000 -s -S

build:
	cargo build --release

strip: build
	$(OBJCOPY) --strip-all $(KERNEL_ELF) -O binary $(KERNEL_BIN)

clean:
	cargo clean

.PHONY: qemu, build, strip, clean