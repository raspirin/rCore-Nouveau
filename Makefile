.PHONY: clean, qemu, build, strip

qemu: build strip
	qemu-system-riscv64 -machine virt -nographic -bios rustsbi-qemu/rustsbi-qemu-release -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000

build:
	cd os && cargo build --release && cd ..

strip:
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin


clean:
	cargo clean