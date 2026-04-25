#ifndef SIGMA_MEMORY_ALLOCATOR_H
#define SIGMA_MEMORY_ALLOCATOR_H

#include <cstddef>
#include <iostream>
#include <vector>

namespace sigma {
namespace core {

// High-Performance Custom Memory Pool Allocator
// Replaces standard malloc/new to prevent heap fragmentation and guarantee O(1) allocation time.
class MemoryPool {
private:
    size_t chunk_size;
    std::vector<void*> free_chunks;
    void* memory_block;

public:
    MemoryPool(size_t size, size_t count) : chunk_size(size) {
        memory_block = ::operator new(size * count);
        char* current = static_cast<char*>(memory_block);
        for (size_t i = 0; i < count; ++i) {
            free_chunks.push_back(current);
            current += size;
        }
        std::cout << "[MemoryPool] Initialized pool: " << count << " chunks of " << size << " bytes." << std::endl;
    }

    ~MemoryPool() {
        ::operator delete(memory_block);
    }

    void* allocate() {
        if (free_chunks.empty()) {
            throw std::bad_alloc();
        }
        void* ptr = free_chunks.back();
        free_chunks.pop_back();
        return ptr;
    }

    void deallocate(void* ptr) {
        free_chunks.push_back(ptr);
    }
};

} // namespace core
} // namespace sigma

#endif // SIGMA_MEMORY_ALLOCATOR_H
