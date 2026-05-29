; =========================================================================
; Σ SIGMAOS: GDT / TSS FLUSH ROUTINES (Phase 16)
; =========================================================================
; Pure assembly GDT load and segment register reload.
; These MUST be in assembly — there is no safe way to reload segment
; registers from C/C++.
;
; void sigma_gdt_load(sigma_gdtr_t* gdtr);
; void sigma_tss_load(sigma_u16 selector);
; void sigma_idt_load(sigma_idtr_t* idtr);
; =========================================================================

[BITS 64]

section .text

; =========================================================================
; sigma_gdt_load(gdtr* rdi)
; =========================================================================
; Loads a new GDT and reloads all segment registers.
; After lgdt, we must do a far return to reload CS, then reload DS/ES/SS.
; =========================================================================
global sigma_gdt_load
sigma_gdt_load:
    lgdt [rdi]

    ; Reload CS via a far return
    ; Push the new CS selector (0x08 = kernel code segment) and return address
    mov  rax, .reload_cs
    push qword 0x08        ; Kernel code segment selector
    push rax
    retfq                  ; Far return: pops RIP and CS

.reload_cs:
    ; Reload data segment registers with kernel data selector (0x10)
    mov  ax, 0x10
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax

    ret


; =========================================================================
; sigma_tss_load(selector di)
; =========================================================================
; Loads the Task State Segment register.
; Must be called after GDT is loaded and TSS descriptor is set up.
; =========================================================================
global sigma_tss_load
sigma_tss_load:
    ltr  di
    ret


; =========================================================================
; sigma_idt_load(idtr* rdi)
; =========================================================================
; Loads the Interrupt Descriptor Table register.
; =========================================================================
global sigma_idt_load
sigma_idt_load:
    lidt [rdi]
    ret
