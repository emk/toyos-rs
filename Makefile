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

libcore_nofp_patch := build/libcore_nofp.patch
libcore_nofp_url := \
	https://raw.githubusercontent.com/thepowersgang/rust-barebones-kernel/master/libcore_nofp.patch
installed_target_libs := \
	~/.multirust/toolchains/nightly/lib/rustlib/$(target)/lib

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
	@cargo rustc --target $(target) -- -Z no-landing-pads

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm $(assembly_header_files)
	@echo NASM $<
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 -Isrc/arch/$(arch)/ $< -o $@


#==========================================================================
# Building the Rust runtime for our bare-metal target

RUSTC := \
	rustc --verbose --target $(target) \
		-Z no-landing-pads \
		--out-dir $(installed_target_libs)

.PHONY: runtime patch core alloc rustc_unicode collections

runtime: core alloc rustc_unicode collections

patch: $(libcore_nofp_patch)
	@echo Patching libcore to remove floating point.
	@(cd rust/src/libcore && patch -p1 < ../../../$(libcore_nofp_patch))

$(libcore_nofp_patch):
	@echo CURL $(libcore_nofp_patch)
	@mkdir -p $(shell dirname $(libcore_nofp_patch))
	@curl -o $(libcore_nofp_patch) $(libcore_nofp_url)

core:
	@echo RUSTC libcore
	@mkdir -p $(installed_target_libs)
	@$(RUSTC) --cfg disable_float rust/src/libcore/lib.rs

alloc:
	@echo RUSTC liballoc
	@mkdir -p $(installed_target_libs)
	@$(RUSTC) rust/src/liballoc/lib.rs

rustc_unicode:
	@echo RUSTC librustc_unicode
	@mkdir -p $(installed_target_libs)
	@$(RUSTC) rust/src/librustc_unicode/lib.rs

collections:
	@echo RUSTC libcollections
	@mkdir -p $(installed_target_libs)
	@$(RUSTC) rust/src/libcollections/lib.rs
