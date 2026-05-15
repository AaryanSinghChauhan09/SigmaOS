#include <core/SigmaOOP.hpp>
#include <core/sigma_types.h>

/**
 * Σ SIGMAOS: Sovereign Memory Pool Manager
 * Implements deterministic shard-level memory pools for O(1) allocation.
 */

class SovereignMemoryPool : public SigmaSingleton<SovereignMemoryPool> {
public:
    struct Pool {
        uintptr_t start;
        size_t size;
        size_t used;
        bool locked;
    };

    void initialize_pool(uintptr_t base, size_t size) {
        m_global_pool = {base, size, 0, false};
    }

    void* allocate(size_t size) {
        if (m_global_pool.used + size > m_global_pool.size) return nullptr;
        
        void* ptr = (void*)(m_global_pool.start + m_global_pool.used);
        m_global_pool.used += size;
        return ptr;
    }

    void reset() {
        m_global_pool.used = 0;
    }

    size_t get_usage_percent() {
        return (m_global_pool.used * 100) / m_global_pool.size;
    }

private:
    Pool m_global_pool;
};

// C-Linkage Shard Interface
extern "C" {
    void* smm_pool_alloc(size_t size) {
        return SovereignMemoryPool::Instance().allocate(size);
    }
    
    void smm_pool_reset() {
        SovereignMemoryPool::Instance().reset();
    }
}
