/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

// SigmaOS Native Memory Allocator (OOP Design)
// ============================================
// Zero dependency. Replaces <stdlib.h> malloc/free.
// Directly interfaces with OS kernel via syscalls (mmap/munmap or VirtualAlloc).

#ifndef SIGMA_MEMORY_ALLOCATOR_HPP
#define SIGMA_MEMORY_ALLOCATOR_HPP

#include "types.h"

#ifdef _WIN32
  // For Windows, we declare the bare minimum from kernel32 via extern
  extern "C" __declspec(dllimport) void* VirtualAlloc(void* lpAddress, size_t dwSize, u32 flAllocationType, u32 flProtect);
  extern "C" __declspec(dllimport) i32 VirtualFree(void* lpAddress, size_t dwSize, u32 dwFreeType);
  #define MEM_COMMIT 0x00001000
  #define MEM_RESERVE 0x00002000
  #define PAGE_READWRITE 0x04
  #define MEM_RELEASE 0x00008000
#else
  // For Linux, raw syscall numbers
  #define SYS_mmap 9
  #define SYS_munmap 11
  #define PROT_READ 0x1
  #define PROT_WRITE 0x2
  #define MAP_PRIVATE 0x02
  #define MAP_ANONYMOUS 0x20
  
  extern "C" void* syscall_mmap(size_t len, int prot, int flags, int fd, size_t offset);
  extern "C" int syscall_munmap(void* addr, size_t len);
#endif

namespace Sigma {
namespace Core {

class MemoryAllocator {
private:
    size_t total_allocated;
    size_t active_allocations;

public:
    MemoryAllocator() : total_allocated(0), active_allocations(0) {}
    ~MemoryAllocator() {} // Clean up metadata if required.

    // Bare metal memory allocation
    void* Allocate(size_t size) {
        if (size == 0) return NULL;
        void* ptr = NULL;

#ifdef _WIN32
        ptr = VirtualAlloc(NULL, size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
#else
        // Direct assembly syscall wrapper or fallback to raw int 0x80 / syscall
        ptr = syscall_mmap(size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
        if ((size_t)ptr == (size_t)-1) ptr = NULL;
#endif

        if (ptr) {
            total_allocated += size;
            active_allocations++;
        }
        return ptr;
    }

    // Bare metal freeing
    void Free(void* ptr, size_t size) {
        if (!ptr) return;

#ifdef _WIN32
        VirtualFree(ptr, 0, MEM_RELEASE);
#else
        syscall_munmap(ptr, size);
#endif
        if (active_allocations > 0) active_allocations--;
        total_allocated -= size;
    }

    // Memory copying (avoids <string.h> memcpy)
    static void Copy(void* dest, const void* src, size_t n) {
        u8* d = (u8*)dest;
        const u8* s = (const u8*)src;
        for (size_t i = 0; i < n; i++) {
            d[i] = s[i];
        }
    }

    // Memory setting (avoids <string.h> memset)
    static void Set(void* dest, u8 val, size_t n) {
        u8* d = (u8*)dest;
        for (size_t i = 0; i < n; i++) {
            d[i] = val;
        }
    }
    
    size_t GetTotalAllocated() const { return total_allocated; }
};

// Global instance for SigmaOS C++ routines
extern MemoryAllocator GlobalAllocator;

} // namespace Core
} // namespace Sigma

// Global overload of new and delete to enforce custom MemoryAllocator globally.
// This bans standard library hidden new/delete.

inline void* operator new(size_t size) {
    return Sigma::Core::GlobalAllocator.Allocate(size);
}
inline void* operator new[](size_t size) {
    return Sigma::Core::GlobalAllocator.Allocate(size);
}
inline void operator delete(void* ptr, size_t size) {
    Sigma::Core::GlobalAllocator.Free(ptr, size);
}
inline void operator delete[](void* ptr, size_t size) {
    Sigma::Core::GlobalAllocator.Free(ptr, size);
}

#endif // SIGMA_MEMORY_ALLOCATOR_HPP

