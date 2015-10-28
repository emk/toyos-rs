;;; Based on http://blog.phil-opp.com/rust-os/multiboot-kernel.html
;;;
;;; The actual boot code of our kernel.

global start

;;; Our main entry point.  Invoked by out boot loader.
section .text
bits 32
start:
        mov esp, stack_top               ; Use our temporary stack.

        ;; Sanity-check our system.
        call test_multiboot
        call test_cpuid
        call test_long_mode

        mov dword [0xb8000], 0x2f4b2f4f  ; Print "OK" to screen.
        hlt

;;; Boot-time error handler.  Prints `ERR: ` and a code.
;;;
;;; al: Error code.
error:
        mov dword [0xb8000], 0x4f524f45
        mov dword [0xb8004], 0x4f3a4f52
        mov dword [0xb8008], 0x4f204f20
        mov byte  [0xb800a], al
        hlt

;;; Make sure we were loaded by multiboot.
test_multiboot:
        cmp eax, 0x36d76289     ; Did multiboot put a magic value in eax?
        je .found_multiboot
        mov al, "M"
        jmp error
.found_multiboot:
        ret

;;; Test for CPUID.  Copied from
;;; http://blog.phil-opp.com/rust-os/entering-longmode.html
;;; which copied from
;;; http://wiki.osdev.org/Setting_Up_Long_Mode#Detection_of_CPUID
test_cpuid:
        pushfd                  ; Store the FLAGS-register.
        pop eax                 ; Restore the A-register.
        mov ecx, eax            ; Set the C-register to the A-register.
        xor eax, 1 << 21        ; Flip the ID-bit, which is bit 21.
        push eax                ; Store the A-register.
        popfd                   ; Restore the FLAGS-register.
        pushfd                  ; Store the FLAGS-register.
        pop eax                 ; Restore the A-register.
        push ecx                ; Store the C-register.
        popfd                   ; Restore the FLAGS-register.
        xor eax, ecx            ; Do a XOR-operation on the A and C.
        jz .no_cpuid            ; The zero flag is set, no CPUID.
        ret                     ; CPUID is available for use.
.no_cpuid:
        mov al, "I"
        jmp error

;;; Test for presence of 64-bit support.  Copied from the same sources as
;;; test_cpuid.
test_long_mode:
        mov eax, 0x80000000     ; Set the A-register to 0x80000000.
        cpuid                   ; CPU identification.
        cmp eax, 0x80000001     ; Compare the A-register with 0x80000001.
        jb .no_long_mode        ; It is less, there is no long mode.
        mov eax, 0x80000001     ; Set the A-register to 0x80000001.
        cpuid                   ; CPU identification.
        ;; Test if the LM-bit, which is bit 29, is set in the D-register.
        test edx, 1 << 29
        jz .no_long_mode        ; They aren't, there is no long mode.
        ret
.no_long_mode:
        mov al, "L"
        jmp error

;;; A tiny stack for our boot loader.
section .bss
stack_bottom:
        resb 64                 ; Bytes to reserve.
stack_top:
