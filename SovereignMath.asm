; =============================================================================
; Σ SIGMAOS: SOVEREIGN MATH (v1.0 - ABSOLUTE FPU FINALITY)
; =============================================================================
; Mission: Zero-MathLib. Zero-Predefined. Pure x87/SSE FPU Logic.
; Capability: Direct Silicon Calculation for Advanced UI/Physics.
; =============================================================================

SECTION .text
    GLOBAL sigma_math_sqrt
    GLOBAL sigma_math_sin
    GLOBAL sigma_math_cos

; sigma_math_sqrt(f64 x): xmm0 = sqrt(xmm0)
sigma_math_sqrt:
    sqrtsd xmm0, xmm0
    ret

; sigma_math_sin(f64 x): fpu_st0 = sin(fpu_st0) -> xmm0
sigma_math_sin:
    sub rsp, 8
    movsd [rsp], xmm0
    fld qword [rsp]
    fsin
    fstp qword [rsp]
    movsd xmm0, [rsp]
    add rsp, 8
    ret

; sigma_math_cos(f64 x): fpu_st0 = cos(fpu_st0) -> xmm0
sigma_math_cos:
    sub rsp, 8
    movsd [rsp], xmm0
    fld qword [rsp]
    fcos
    fstp qword [rsp]
    movsd xmm0, [rsp]
    add rsp, 8
    ret

