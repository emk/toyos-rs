# Copied from http://blog.phil-opp.com/rust-os/multiboot-kernel.html

arch ?= x86_64
target ?= $(arch)-unknown-none-gnu

rust_os := target/$(target)/debug/libtoyos.a
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_header_files := $(wildcard src/arch/$(arch)/*.inc)
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all fmt clean run debug iso cargo

all: $(kernel)

fmt:
	rustfmt --write-mode overwrite src/lib.rs

clean:
	rm -rf build target

run: $(iso)
	@echo QEMU $(iso)
	@qemu-system-x86_64 -hda $(iso) -serial stdio

debug: $(iso)
	@echo QEMU -d int $(iso)
	@qemu-system-x86_64 -hda $(iso) -d int -no-reboot -serial stdio

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
	@cargo build --target $(target)

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm $(assembly_header_files)
	@echo NASM $<
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 -Isrc/arch/$(arch)/ $< -o $@


#==========================================================================
# Building the Rust runtime for our bare-metal target

# Where to put our compiled runtime libraries for this platform.
installed_target_libs := \
	$(shell multirust which rustc | \
		sed s,bin/rustc,lib/rustlib/$(target)/lib,)

runtime_rlibs := \
	$(installed_target_libs)/libcore.rlib \
	$(installed_target_libs)/liballoc.rlib \
	$(installed_target_libs)/librustc_unicode.rlib \
	$(installed_target_libs)/libcollections.rlib

RUSTC := \
	rustc --verbose --target $(target) \
		-Z no-landing-pads \
		--cfg disable_float \
		--out-dir $(installed_target_libs)

.PHONY: runtime 

runtime: $(runtime_rlibs)

$(installed_target_libs):
	@mkdir -p $(installed_target_libs)

$(installed_target_libs)/%.rlib: rust/src/%/lib.rs $(installed_target_libs)
	@echo RUSTC $<
	@$(RUSTC) $<
	@echo Check $(installed_target_libs)
