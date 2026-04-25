// SigmaOS — sigma-libc-string: Native String Operations
// Modularised from: SovereignLibC.c
// USP: OOP string operations avoiding undefined behavior paths

#ifndef SIGMA_LIBC_STRING_HPP
#define SIGMA_LIBC_STRING_HPP

namespace sigma {
namespace libc {

class StringOps {
public:
    virtual ~StringOps() = default;

    virtual unsigned long strlen(const char* str) const {
        unsigned long len = 0;
        while (str[len]) len++;
        return len;
    }

    virtual int strcmp(const char* s1, const char* s2) const {
        while (*s1 && (*s1 == *s2)) {
            s1++;
            s2++;
        }
        return *(const unsigned char*)s1 - *(const unsigned char*)s2;
    }

    virtual char* strcpy(char* dest, const char* src) {
        char* d = dest;
        while ((*d++ = *src++));
        return dest;
    }
};

// Safe string operations extending base StringOps
class SafeStringOps : public StringOps {
public:
    // Always null-terminates, guarantees no overflow
    char* strncpy(char* dest, const char* src, unsigned long n) {
        if (n == 0) return dest;
        unsigned long i = 0;
        for (; i < n - 1 && src[i] != '\0'; i++) {
            dest[i] = src[i];
        }
        dest[i] = '\0';
        return dest;
    }

    // Bounded string concatenation
    char* strncat(char* dest, const char* src, unsigned long dest_sz) {
        unsigned long dlen = this->strlen(dest);
        if (dlen >= dest_sz - 1) return dest;
        
        unsigned long i = 0;
        unsigned long max_copy = dest_sz - dlen - 1;
        while (src[i] != '\0' && i < max_copy) {
            dest[dlen + i] = src[i];
            i++;
        }
        dest[dlen + i] = '\0';
        return dest;
    }
};

} // namespace libc
} // namespace sigma

#endif // SIGMA_LIBC_STRING_HPP
