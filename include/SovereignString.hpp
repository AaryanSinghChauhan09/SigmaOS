#ifndef SOVEREIGN_STRING_HPP
#define SOVEREIGN_STRING_HPP

#include "sigma_types.h"
#include "SovereignLibC.h"

namespace SigmaOS {
namespace Core {

/*
 * =========================================================================
 * SOVEREIGN STRING (Zero-Dependency / Industrial-Grade)
 * =========================================================================
 * A memory-safe, low-level string abstraction designed for the 
 * Sovereign Kernel. Avoids dynamic allocation overhead where possible.
 */
class SovereignString {
private:
    char* m_buffer;
    sigma_size_t m_length;
    sigma_size_t m_capacity;

public:
    SovereignString() : m_buffer(SIGMA_NULL), m_length(0), m_capacity(0) {}
    
    SovereignString(const char* s) {
        m_length = sigma_strlen(s);
        m_capacity = m_length + 1;
        m_buffer = (char*)sigma_malloc(m_capacity);
        if (m_buffer) {
            sigma_strcpy(m_buffer, s);
        }
    }

    ~SovereignString() {
        if (m_buffer) {
            sigma_free(m_buffer);
        }
    }

    // Explicit Copy Logic (Industrial Standard)
    SovereignString(const SovereignString& other) {
        m_length = other.m_length;
        m_capacity = other.m_capacity;
        m_buffer = (char*)sigma_malloc(m_capacity);
        if (m_buffer && other.m_buffer) {
            sigma_memcpy(m_buffer, other.m_buffer, m_capacity);
        }
    }

    const char* c_str() const { return m_buffer ? m_buffer : ""; }
    sigma_size_t length() const { return m_length; }

    sigma_bool operator==(const char* s) const {
        return (sigma_strcmp(c_str(), s) == 0) ? SIGMA_TRUE : SIGMA_FALSE;
    }
};

} // namespace Core
} // namespace SigmaOS

#endif
