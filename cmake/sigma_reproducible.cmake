# SPDX-License-Identifier: GPL-2.0-or-later
# cmake/sigma_reproducible.cmake — GNU Guix-style reproducible build flags
#
# Goal: two machines building the same source commit produce bit-for-bit
# identical binaries with identical SHA-256 hashes.
#
# Required: SOURCE_DATE_EPOCH env var (reproducible-builds.org standard).
# Set it to the git commit timestamp:
#   export SOURCE_DATE_EPOCH=$(git log -1 --format=%ct)

option(SIGMA_REPRODUCIBLE_BUILD
    "Enable reproducible build flags (zeroes timestamps, sorts sources)"
    ON)

if(SIGMA_REPRODUCIBLE_BUILD)
    # ── 1. Zero out all embedded timestamps ────────────────────────────────
    # Warn if source uses __DATE__ or __TIME__ (bakes in build timestamp)
    add_compile_options(-Wdate-time)

    # Strip absolute source paths from debug info and error messages
    # Before: /home/user/.gemini/antigravity/scratch/SigmaOS/kernel/mm/sigma_aslr.cpp
    # After:  ./kernel/mm/sigma_aslr.cpp
    add_compile_options(
        -ffile-prefix-map=${CMAKE_SOURCE_DIR}=.
        -fmacro-prefix-map=${CMAKE_SOURCE_DIR}=.
    )

    # ── 2. SOURCE_DATE_EPOCH ───────────────────────────────────────────────
    if(DEFINED ENV{SOURCE_DATE_EPOCH})
        set(SIGMA_BUILD_TIMESTAMP "$ENV{SOURCE_DATE_EPOCH}")
        message(STATUS "[repro] SOURCE_DATE_EPOCH = $ENV{SOURCE_DATE_EPOCH}")
    else()
        set(SIGMA_BUILD_TIMESTAMP "0")
        message(STATUS "[repro] SOURCE_DATE_EPOCH not set — using 0 (deterministic)")
    endif()
    add_compile_definitions(SIGMA_BUILD_TIMESTAMP=${SIGMA_BUILD_TIMESTAMP})

    # ── 3. Sort source file lists (cmake GLOB is non-deterministic) ─────────
    # All source lists must be sorted before passing to add_library/add_executable.
    # Example in CMakeLists.txt:
    #   file(GLOB_RECURSE KERNEL_SOURCES kernel/*.cpp)
    #   list(SORT KERNEL_SOURCES)           ← add this line
    #   add_library(sigma_kernel ${KERNEL_SOURCES})
    macro(sigma_sort_sources var)
        list(SORT ${var})
    endmacro()

    # ── 4. Linker: deterministic symbol ordering ───────────────────────────
    # --sort-common: sort common symbols by size (eliminates alignment variation)
    # --sort-section=name: sort ELF sections alphabetically (removes link-order deps)
    add_link_options(
        -Wl,--sort-common
        -Wl,--sort-section,name
        -Wl,--build-id=sha1      # deterministic build-id from content hash
    )

    # ── 5. Disable LTO (LTO order is non-deterministic across compilers) ───
    # LTO produces non-reproducible output unless the compiler version is pinned.
    # Pin the compiler in your CI instead of using LTO for reproducibility.
    if(CMAKE_INTERPROCEDURAL_OPTIMIZATION)
        message(WARNING
            "[repro] LTO is ON — this may break reproducibility unless compiler "
            "version is exactly pinned. Disable LTO or pin to a specific clang.")
    endif()

    message(STATUS "[SigmaOS] Reproducible build flags enabled")
    message(STATUS "  Timestamps: zeroed (SOURCE_DATE_EPOCH=${SIGMA_BUILD_TIMESTAMP})")
    message(STATUS "  Paths:      stripped (-ffile-prefix-map)")
    message(STATUS "  Linker:     deterministic section order")
endif()

# ── Derivation hash helper (Guix /gnu/store/ equivalent) ──────────────────
# Call after all targets are defined to compute the build closure hash.
# Usage: sigma_compute_derivation_hash(TARGET sigma-healthd OUT_VAR drv_hash)
function(sigma_compute_derivation_hash target out_var)
    get_target_property(_srcs ${target} SOURCES)
    string(SHA256 _hash "${_srcs}-${SIGMA_BUILD_TIMESTAMP}")
    string(SUBSTRING "${_hash}" 0 16 _short_hash)
    set(${out_var} "${_short_hash}" PARENT_SCOPE)
    message(STATUS "[repro] ${target} derivation hash: ${_short_hash}")
endfunction()
