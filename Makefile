# Copied from http://blog.phil-opp.com/rust-os/multiboot-kernel.html

arch ?= x86_64
target ?= $(arch)-unknown-linux-gnu

rust_os := target/$(target)/debug/libtoyos.a
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_header_files := $(wildcard src/arch/$(arch)/*.inc)
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run debug iso cargo

all: $(kernel)

clean:
	rm -rf build

run: $(iso)
	@echo QEMU $(iso)
	@qemu-system-x86_64 -hda $(iso)

debug: $(iso)
	@echo QEMU -d int $(iso)
	@qemu-system-x86_64 -hda $(iso) -d int -no-reboot

$(iso): $(kernel) $(grub_cfg)
	@echo ISO $(iso)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue /usr/lib/grub/i386-pc -o $(iso) build/isofiles \
		2> /dev/null
	@rm -r build/isofiles

$(kernel): cargo $(assembly_object_files) $(linker_script)
	@echo LD $(kernel)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) \
		$(assembly_object_files) $(rust_os)

cargo:
	@echo CARGO
	@cargo rustc --target $(target) -- -Z no-landing-pads -C no-redzone

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm $(assembly_header_files)
	@echo NASM $<
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 -Isrc/arch/$(arch)/ $< -o $@
