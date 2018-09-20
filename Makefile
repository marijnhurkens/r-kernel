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

qemu-gdb: build
	qemu-system-x86_64.exe -s -S -drive format=raw,file=target/x86_64-rust_kernel/debug/bootimage-rust_kernel.bin -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04

gdb:
	gdb ./target/x86_64-rust_kernel/debug/rust_kernel -ex 'set arch i386:x86-64:intel' -ex 'target remote localhost:1234' -ex 'break _start' -ex 'cont'

.PHONY: all qemu qemu-test gdb
