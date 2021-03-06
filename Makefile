all: build qemu

test: 
	cargo test
	bootimage test

build:
	cargo bootimage

qemu-test:
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_kernel/debug/bootimage-rust_kernel.bin -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none

qemu:
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_kernel/debug/bootimage-rust_kernel.bin -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04

qemu-gdb: build
	qemu-system-x86_64 -s -S -drive format=raw,file=target/x86_64-rust_kernel/debug/bootimage-rust_kernel.bin -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04

gdb:
	gdb ./target/x86_64-rust_kernel/debug/rust_kernel -ex 'set arch i386:x86-64:intel' -ex 'target remote localhost:1234' -ex 'break _start' -ex 'checkpoint' -ex 'cont'

.PHONY: all test build qemu qemu-test qemu-gdb gdb
