// SigmaOS Native String Class (OOP Design)
// ===========================================
// Zero dependency. Replaces <string> and <string.h>.
// Designed specifically for automation and personalisation strings.

#ifndef SIGMA_STRING_HPP
#define SIGMA_STRING_HPP

#include "types.h"
#include "MemoryAllocator.hpp"

namespace Sigma {
namespace Core {

class String {
private:
    char* data;
    size_t length;
    size_t capacity;

    void AllocSpace(size_t new_cap) {
        if (new_cap <= capacity) return;
        char* new_data = (char*)GlobalAllocator.Allocate(new_cap);
        if (data) {
            MemoryAllocator::Copy(new_data, data, length);
            GlobalAllocator.Free(data, capacity);
        }
        data = new_data;
        capacity = new_cap;
    }

public:
    String() : data(NULL), length(0), capacity(0) {}

    String(const char* c_str) {
        length = 0;
        while (c_str[length] != '\0') length++;
        capacity = length + 1;
        data = (char*)GlobalAllocator.Allocate(capacity);
        MemoryAllocator::Copy(data, c_str, length);
        data[length] = '\0';
    }

    String(const String& other) {
        length = other.length;
        capacity = other.capacity;
        data = (char*)GlobalAllocator.Allocate(capacity);
        MemoryAllocator::Copy(data, other.data, length + 1);
    }

    ~String() {
        if (data) {
            GlobalAllocator.Free(data, capacity);
        }
    }

    String& operator=(const String& other) {
        if (this != &other) {
            if (data) GlobalAllocator.Free(data, capacity);
            length = other.length;
            capacity = other.capacity;
            data = (char*)GlobalAllocator.Allocate(capacity);
            MemoryAllocator::Copy(data, other.data, length + 1);
        }
        return *this;
    }

    String operator+(const String& other) const {
        String res;
        res.AllocSpace(length + other.length + 1);
        MemoryAllocator::Copy(res.data, data, length);
        MemoryAllocator::Copy(res.data + length, other.data, other.length);
        res.length = length + other.length;
        res.data[res.length] = '\0';
        return res;
    }

    bool operator==(const String& other) const {
        if (length != other.length) return false;
        for (size_t i = 0; i < length; i++) {
            if (data[i] != other.data[i]) return false;
        }
        return true;
    }

    const char* c_str() const { return data ? data : ""; }
    size_t Length() const { return length; }

    // Custom helper for Linux distro absorber
    bool Contains(const char* substr) const {
        if (!data || !substr) return false;
        size_t sub_len = 0;
        while (substr[sub_len] != '\0') sub_len++;
        if (sub_len > length) return false;

        for (size_t i = 0; i <= length - sub_len; i++) {
            bool match = true;
            for (size_t j = 0; j < sub_len; j++) {
                if (data[i + j] != substr[j]) {
                    match = false;
                    break;
                }
            }
            if (match) return true;
        }
        return false;
    }
};

} // namespace Core
} // namespace Sigma

#endif // SIGMA_STRING_HPP
