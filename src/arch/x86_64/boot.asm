;;; Based on http://blog.phil-opp.com/rust-os/multiboot-kernel.html
;;;
;;; The actual boot code of our kernel.

global start

section .text
bits 32
start:
        mov dword [0xb8000], 0x2f4b2f4f  ; Print "OK" to screen.
        hlt
