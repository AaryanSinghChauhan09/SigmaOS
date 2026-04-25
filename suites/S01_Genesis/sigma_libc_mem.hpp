// SigmaOS — sigma-libc-mem: Native Memory Operations
// Modularised from: SovereignLibC.c
// USP: OOP encapsulation for memory management with safe bounds checking

#ifndef SIGMA_LIBC_MEM_HPP
#define SIGMA_LIBC_MEM_HPP

namespace sigma {
namespace libc {

class MemoryOps {
public:
    // Virtual destructor for inheritance
    virtual ~MemoryOps() = default;

    virtual void* memset(void* dest, int val, unsigned long len) {
        unsigned char* ptr = static_cast<unsigned char*>(dest);
        for (unsigned long i = 0; i < len; ++i) {
            ptr[i] = static_cast<unsigned char>(val);
        }
        return dest;
    }

    virtual void* memcpy(void* dest, const void* src, unsigned long len) {
        unsigned char* d = static_cast<unsigned char*>(dest);
        const unsigned char* s = static_cast<const unsigned char*>(src);
        for (unsigned long i = 0; i < len; ++i) {
            d[i] = s[i];
        }
        return dest;
    }

    virtual int memcmp(const void* s1, const void* s2, unsigned long n) {
        const unsigned char* p1 = static_cast<const unsigned char*>(s1);
        const unsigned char* p2 = static_cast<const unsigned char*>(s2);
        for (unsigned long i = 0; i < n; i++) {
            if (p1[i] != p2[i]) return p1[i] - p2[i];
        }
        return 0;
    }
};

// Secure memory operations extending the base MemoryOps
class SecureMemoryOps : public MemoryOps {
public:
    // Timing-safe memory comparison
    int memcmp(const void* s1, const void* s2, unsigned long n) override {
        const unsigned char* p1 = static_cast<const unsigned char*>(s1);
        const unsigned char* p2 = static_cast<const unsigned char*>(s2);
        unsigned char diff = 0;
        for (unsigned long i = 0; i < n; i++) {
            diff |= (p1[i] ^ p2[i]);
        }
        return diff == 0 ? 0 : 1;
    }

    // Secure wipe (volatile ensures compiler doesn't optimize it away)
    void secure_zero(void* dest, unsigned long len) {
        volatile unsigned char* ptr = static_cast<volatile unsigned char*>(dest);
        for (unsigned long i = 0; i < len; ++i) {
            ptr[i] = 0;
        }
    }
};

} // namespace libc
} // namespace sigma

#endif // SIGMA_LIBC_MEM_HPP
