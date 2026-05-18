/**
 * SigmaOS: Sovereign Win32 Compatibility Layer (S32-Sim)
 * Inspired by ReactOS.
 * USP: Allow legacy application headers to compile within the Sovereign Lattice.
 */

#ifndef SIGMA_WIN32_SIM_H
#define SIGMA_WIN32_SIM_H

#include "libc/sigma_libc.h"

typedef void* HANDLE;
typedef uint32_t DWORD;
typedef int BOOL;
typedef char* LPSTR;

#define TRUE  1
#define FALSE 0

// Mock Win32 APIs redirected to SigmaOS Shards
static inline HANDLE CreateFileA(LPSTR name, DWORD access, DWORD share, void* sec, DWORD disp, DWORD flags, HANDLE templ) {
    // Redirect to S06_Storage Shard
    return (HANDLE)0x1;
}

static inline BOOL WriteFile(HANDLE file, const void* buffer, DWORD len, DWORD* written, void* overlap) {
    // Redirect to S06_Storage Shard
    return TRUE;
}

static inline void ExitProcess(uint32_t code) {
    // Redirect to S03_Orchestrator
}

#endif // SIGMA_WIN32_SIM_H
