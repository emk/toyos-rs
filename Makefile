# Copied from http://blog.phil-opp.com/rust-os/multiboot-kernel.html

arch ?= x86_64

kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).bin

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
    build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso

all: $(kernel)

clean:
	rm -rf build

run: $(iso)
	@qemu-system-x86_64 -hda $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p build/isofiles/boot/grub
	cp $(kernel) build/isofiles/boot/kernel.bin
	cp $(grub_cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles
	rm -r build/isofiles

$(kernel): $(assembly_object_files) $(linker_script)
	ld -n -T $(linker_script) -o $(kernel) $(assembly_object_files)

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@
