# SPDX-License-Identifier: GPL-2.0-or-later
# sigma_hardening.cmake — Chimera Linux-style typed hardening registry
#
# Usage:
#   include(cmake/sigma_hardening.cmake)
#   sigma_apply_hardening(sigma-apid)
#   sigma_apply_hardening(sigma-trustd)
#
# Override default set on command line:
#   cmake -DSIGMA_HARDENING="vis;ssp;pie;relro" -B build  # no CFI (GCC build)
#   cmake -DSIGMA_HARDENING="vis;cfi;ssp;pie;relro;nortti" -B build  # Clang full

set(SIGMA_HARDENING "vis;cfi;ssp;pie;relro" CACHE STRING
    "Hardening flags: vis cfi ssp pie relro nortti safestack")

function(sigma_apply_hardening target)
    set(harden_cflags "")
    set(harden_ldflags "")

    # ── vis: hidden symbol visibility (Chimera "vis") ──────────────────────
    if("vis" IN_LIST SIGMA_HARDENING)
        list(APPEND harden_cflags
            -fvisibility=hidden
            -fvisibility-inlines-hidden)
    endif()

    # ── cfi: Control Flow Integrity (Chimera "cfi", Clang only) ────────────
    if("cfi" IN_LIST SIGMA_HARDENING)
        if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang" OR
           CMAKE_C_COMPILER_ID   STREQUAL "Clang")
            list(APPEND harden_cflags
                -fsanitize=cfi-icall          # indirect calls
                -fsanitize=cfi-vcall          # virtual dispatch
                -fsanitize=cfi-nvcall         # non-virtual method calls
                -fsanitize=cfi-unrelated-cast # type-confusion casts
                -fno-sanitize-trap=cfi        # report, don't silently trap
                -flto                         # CFI requires LTO
            )
            list(APPEND harden_ldflags -fsanitize=cfi -flto)
        else()
            message(STATUS "[hardening] CFI skipped for ${target}: requires Clang")
        endif()
    endif()

    # ── ssp: stack smash protection ────────────────────────────────────────
    if("ssp" IN_LIST SIGMA_HARDENING)
        list(APPEND harden_cflags -fstack-protector-strong)
    endif()

    # ── pie: position-independent executable ───────────────────────────────
    if("pie" IN_LIST SIGMA_HARDENING)
        list(APPEND harden_cflags -fPIE)
        list(APPEND harden_ldflags -pie)
    endif()

    # ── relro: read-only relocations ───────────────────────────────────────
    if("relro" IN_LIST SIGMA_HARDENING)
        list(APPEND harden_ldflags -Wl,-z,relro -Wl,-z,now)
    endif()

    # ── nortti: disable RTTI (Chimera dinit) ───────────────────────────────
    if("nortti" IN_LIST SIGMA_HARDENING)
        list(APPEND harden_cflags -fno-rtti)
    endif()

    # ── safestack: LLVM SafeStack (HardenedBSD) ────────────────────────────
    if("safestack" IN_LIST SIGMA_HARDENING)
        if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
            list(APPEND harden_cflags -fsanitize=safe-stack)
            list(APPEND harden_ldflags -fsanitize=safe-stack)
        endif()
    endif()

    # ── Apply ──────────────────────────────────────────────────────────────
    if(harden_cflags)
        target_compile_options(${target} PRIVATE ${harden_cflags})
    endif()
    if(harden_ldflags)
        target_link_options(${target} PRIVATE ${harden_ldflags})
    endif()

    message(STATUS "[hardening] ${target}: ${SIGMA_HARDENING}")
endfunction()

# Convenience: apply to all known sigma userland targets
macro(sigma_harden_all_targets)
    set(_sigma_userland_targets
        sigma-apid sigma-trustd sigma-healthd sigma-vault
        sigma-search sigma-webhook sigma-ds sigma-indexd
        sigma-pkg-cli sigma-sysctl-cli sigmactl)
    foreach(_t IN LISTS _sigma_userland_targets)
        if(TARGET ${_t})
            sigma_apply_hardening(${_t})
        endif()
    endforeach()
endmacro()
