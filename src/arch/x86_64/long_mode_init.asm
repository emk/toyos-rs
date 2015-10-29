;;; Based on http://blog.phil-opp.com/rust-os/entering-longmode.html
;;;
;;; Once we've run all our 32-bit setup code, we jump here and enter 64-bit
;;; mode.

global long_mode_start

SCREEN_BASE equ 0xb8000

section .text
bits 64
long_mode_start:
        ;; Display "OKAY".
        mov rax, 0x2f592f412f4b2f4f
        mov qword [SCREEN_BASE], rax
        hlt

