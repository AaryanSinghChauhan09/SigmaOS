/*
 * Σ SIGMA OS: SOVEREIGN C++ STL (v7.0 - NO-STD C++)
 * ==========================================================
 * USP Absorbed: KDevelop / Custom Engine Architecture (UE5).
 * Capability: Custom C++ STL implementation. No <vector>, no <string>, no <memory>.
 * Principle: Only custom Sigma allocating mechanisms from SigmaLibC.
 */

#ifndef SIGMACPPS_H
#define SIGMACPPS_H

#include "SigmaLibC.h"

// ---------------------------------------------------------
// SigmaVector (Custom Dynamic Array)
// Replaces std::vector
// ---------------------------------------------------------
template <typename T>
class SigmaVector {
private:
    T* m_data;
    sigma_u64 m_size;
    sigma_u64 m_capacity;

    // Direct mapping to the Sigma Static Pool (Zero-Malloc)
    // For demonstration, simulating a static shard pool wrapper:
    T* SigmaAllocate(sigma_u64 count) {
        // Here we would hook into SigmaZeroLibMemory's sigma_native_alloc
        // For safe compilation logic simulation without linkage errors, we declare it:
        extern void* sigma_native_alloc(sigma_u64 size);
        return (T*)sigma_native_alloc(count * sizeof(T));
    }

public:
    SigmaVector() : m_data(nullptr), m_size(0), m_capacity(0) {}
    
    ~SigmaVector() { /* Handled by amnesic wipe */ }

    void Push(const T& value) {
        if (m_size >= m_capacity) {
            sigma_u64 new_cap = (m_capacity == 0) ? 8 : m_capacity * 2;
            T* new_data = SigmaAllocate(new_cap);
            if (m_data) {
                sigma_memcpy(new_data, m_data, m_size * sizeof(T));
                // Previous memory is wiped by system pool reset, not generic free.
            }
            m_data = new_data;
            m_capacity = new_cap;
        }
        m_data[m_size++] = value;
    }

    sigma_u64 Size() const { return m_size; }
    T& operator[](sigma_u64 index) { return m_data[index]; }
};

// ---------------------------------------------------------
// SigmaString (Custom Fixed String Shard)
// Replaces std::string
// ---------------------------------------------------------
class SigmaString {
private:
    char m_buffer[128]; // Fixed stack-string
    sigma_u64 m_length;

public:
    SigmaString(const char* str) {
        m_length = 0;
        while(str[m_length] && m_length < 127) {
            m_buffer[m_length] = str[m_length];
            m_length++;
        }
        m_buffer[m_length] = '\0';
    }

    const char* CStr() const { return m_buffer; }
    
    void Print() const {
        sigma_print(m_buffer);
    }
};

#endif // SIGMACPPS_H
