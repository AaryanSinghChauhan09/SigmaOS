/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN OOP FRAMEWORK (v19.0 - ZERO-DEPENDENCY)
 * =========================================================================
 * Mission: Extreme HLL Dependency Reduction (No Stdlib, No Iostream).
 * Capability: Ring-0 OOP via custom vtable sharding.
 * Principle: Bit-Perfect. Silicon-Direct. Zero-Dependency.
 * =========================================================================
 */

#ifndef SIGMA_OOP_HPP
#define SIGMA_OOP_HPP

#include "SovereignLibC.h"

namespace SigmaOS {

// --- Core Types (Low-Level Zenith) ---
typedef sigma_u32 sigma_status;
#define SIGMA_OK    0x00000000
#define SIGMA_ERROR 0xFFFFFFFF

// --- Sovereign Memory Management (Direct Syscalls) ---
class SigmaMemory {
public:
    static void* allocate(sigma_size_t length) {
        // Map direct shard memory via sigma_mmap (Syscall 9)
        return sigma_mmap(0, length, 3, 0x22, -1, 0); 
    }
};

// --- Sovereign Object Model (The Shard) ---
class SigmaObject {
public:
    virtual ~SigmaObject() = default;
    virtual const char* type_name() const noexcept = 0;
};

// --- Sovereign String Shard ---
class SigmaString {
private:
    char*        m_data;
    sigma_size_t m_len;
public:
    SigmaString(const char* s = "") {
        m_len = sigma_strlen(s);
        m_data = (char*)sigma_malloc(m_len + 1);
        sigma_memcpy(m_data, s, m_len + 1);
    }
    void append(const char* s) {
        sigma_size_t slen = sigma_strlen(s);
        char* next = (char*)sigma_malloc(m_len + slen + 1);
        sigma_memcpy(next, m_data, m_len);
        sigma_memcpy(next + m_len, s, slen + 1);
        m_data = next;
        m_len += slen;
    }
    const char* c_str() const { return m_data; }
};

// --- Sovereign Map Shard ---
template<typename K, typename V>
class SigmaMap {
private:
    K m_keys[64];
    V m_values[64];
    sigma_size_t m_size;
public:
    SigmaMap() : m_size(0) {}
    void insert(const K& key, const V& value) {
        if (m_size < 64) {
            m_keys[m_size] = key;
            m_values[m_size] = value;
            m_size++;
        }
    }
    sigma_size_t size() const { return m_size; }
    const K& key_at(sigma_size_t index) const { return m_keys[index]; }
    const V* at_index(sigma_size_t index) const { return &m_values[index]; }
};

#if defined(__x86_64__) || defined(_M_X64)
    #define SIGMA_ARCH_X86_64
#endif

extern "C" {
    int sigma_snprintf(char* str, sigma_size_t size, const char* format, ...);
}

} // namespace SigmaOS

/* Global overrides for zero-dependency C++ support */
inline void* operator new(sigma_size_t size) {
    return sigma_malloc(size);
}

inline void* operator new[](sigma_size_t size) {
    return sigma_malloc(size);
}

inline void operator delete(void* ptr) noexcept {
    sigma_free(ptr);
}

inline void operator delete(void* ptr, sigma_size_t size) noexcept {
    (void)size;
    sigma_free(ptr);
}

inline void operator delete[](void* ptr) noexcept {
    sigma_free(ptr);
}

#endif
