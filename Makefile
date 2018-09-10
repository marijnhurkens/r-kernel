all: build qemu

test: 
	cargo test
	bootimage test

build:
	bootimage build

qemu-test:
	qemu-system-x86_64.exe -drive format=raw,file=target/x86_64-rust_kernel/debug/bootimage-rust_kernel.bin -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none

qemu:
	qemu-system-x86_64.exe -drive format=raw,file=target/x86_64-rust_kernel/debug/bootimage-rust_kernel.bin -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04

.PHONY: all qemu qemu-test
