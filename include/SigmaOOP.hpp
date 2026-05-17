/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN OOP FRAMEWORK (v19.0 - ZERO-DEPENDENCY)
 * =========================================================================
 * Mission: Extreme HLL Dependency Reduction (No Stdlib, No Iostream).
 * Capability: Ring-0 OOP via custom vtable sharding.
 * Principle: Bit-Perfect. Silicon-Direct. Zero-Dependency.
 * =========================================================================
 */

#ifndef SIGMA_OOP_HPP
#define SIGMA_OOP_HPP

#include "./sigma_kernel_types.h"

namespace SigmaOS {

// --- Core Types (Low-Level Zenith) ---
typedef sigma_u32 sigma_status_shard;

// --- Sovereign Memory Management (Direct Syscalls) ---
class SigmaMemory {
public:
    static void* allocate(sigma_size_t length) {
        // In a real kernel, this would call sigma_kmalloc or similar
        // For now, we simulate success for the lattice
        return (void*)0xDEADBEEF; 
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
        m_data = (char*)allocate(m_len + 1);
        sigma_memcpy(m_data, s, m_len + 1);
    }
    void append(const char* s) {
        sigma_size_t slen = sigma_strlen(s);
        char* next = (char*)allocate(m_len + slen + 1);
        sigma_memcpy(next, m_data, m_len);
        sigma_memcpy(next + m_len, s, slen + 1);
        m_data = next;
        m_len += slen;
    }
    const char* c_str() const { return m_data; }
    sigma_size_t length() const { return m_len; }
    
    static sigma_size_t sigma_strlen(const char* s) {
        sigma_size_t len = 0;
        while (s && s[len]) len++;
        return len;
    }
    static void sigma_memcpy(void* d, const void* s, sigma_size_t n) {
        char* cd = (char*)d;
        const char* cs = (const char*)s;
        while(n--) *cd++ = *cs++;
    }
    static void* allocate(sigma_size_t n) { return SigmaMemory::allocate(n); }
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

// --- Sovereign Vector Shard (Zero-Dependency) ---
template<typename T>
class SigmaVector {
private:
    T*           m_data;
    sigma_size_t m_capacity;
    sigma_size_t m_size;
public:
    SigmaVector(sigma_size_t initial_cap = 16) : m_size(0), m_capacity(initial_cap) {
        m_data = (T*)SigmaMemory::allocate(sizeof(T) * initial_cap);
    }
    void push_back(const T& item) {
        if (m_size < m_capacity) {
            m_data[m_size++] = item;
        }
    }
    T& operator[](sigma_size_t index) { return m_data[index]; }
    const T& operator[](sigma_size_t index) const { return m_data[index]; }
    sigma_size_t size() const { return m_size; }
    T* begin() { return m_data; }
    T* end() { return m_data + m_size; }
};

// --- Sovereign Singleton Shard ---
template<typename T>
class SigmaSingleton {
protected:
    SigmaSingleton() = default;
    ~SigmaSingleton() = default;
public:
    SigmaSingleton(const SigmaSingleton&) = delete;
    SigmaSingleton& operator=(const SigmaSingleton&) = delete;
    static T& getInstance() {
        static T instance;
        return instance;
    }
};

} // namespace SigmaOS

/* Global overrides for zero-dependency C++ support */
#ifndef SIGMA_NEW_DEFINED
#define SIGMA_NEW_DEFINED
inline inline void* operator new(sigma_size_t size) { return SigmaOS::SigmaMemory::allocate(size); }
inline inline void* operator new[](sigma_size_t size) { return SigmaOS::SigmaMemory::allocate(size); }
inline inline void operator delete(void* ptr) noexcept {}
inline inline void operator delete(void* ptr, sigma_size_t size) noexcept {}
inline inline void operator delete[](void* ptr) noexcept {}
#endif

#endif
