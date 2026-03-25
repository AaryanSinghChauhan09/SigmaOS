#ifndef SOVEREIGN_SHARD_HPP
#define SOVEREIGN_SHARD_HPP

#include <iostream>
#include <string>
#include <vector>

/**
 * SigmaOS Sovereign Shard Base (Native C++ OOPS)
 * Principle: OOPS, SOLID, Data Encapsulation.
 * USP: Polymorphic Hardware/Shard Interaction.
 * Inspiration: torvalds/linux/include/linux/fs.h (File Operations OOPS-like pointers).
 */

namespace SigmaOS {

    class IShard {
    public:
        virtual ~IShard() {}
        virtual uint32_t GetId() const = 0;
        virtual std::string GetName() const = 0;
        virtual void Execute() = 0;
        virtual void StatusReport() {
            std::cout << "Σ [CPP_SHARD]: OK: Shard [" << GetName() << "] Operational." << std::endl;
        }
    };

    class MemoryShard : public IShard {
    private:
        uint32_t m_id;
        std::string m_name;
    public:
        MemoryShard(uint32_t id, std::string name) : m_id(id), m_name(name) {}
        uint32_t GetId() const override { return m_id; }
        std::string GetName() const override { return m_name; }
        void Execute() override {
            std::cout << "Σ [CPP_SHARD]: Memory Shard Dispatch: " << m_name << std::endl;
        }
    };

} // namespace SigmaOS

#endif
