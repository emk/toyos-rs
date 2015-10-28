;;; Based on http://blog.phil-opp.com/rust-os/multiboot-kernel.html
;;;
;;; This is our Multiboot 2 header, which Grub uses to find our kernel code
;;; and load it into memory.

MULTIBOOT_MAGIC equ 0xe85250d6        ; Magic number for multiboot 2.
ARCHITECTURE    equ 0                 ; Protected mode i386 architecture.

section .multiboot_header
header_start:
        dd MULTIBOOT_MAGIC            ; Magic.
        dd ARCHITECTURE               ; Architecture.
        dd header_end - header_start  ; Length.
        ;; Checksum.
        dd 0x100000000 - (MULTIBOOT_MAGIC + ARCHITECTURE + (header_end - header_start))

        ;; Multiboot tags.

        ;; End tag.
        dw 0                          ; Type.
        dw 0                          ; Flags.
        dd 8                          ; Size.
header_end:
