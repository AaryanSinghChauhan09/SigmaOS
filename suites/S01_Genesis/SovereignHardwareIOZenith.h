#include "../../include/sigma_types.h"
#ifndef SOVEREIGN_HARDWARE_IO_ZENITH_H
#define SOVEREIGN_HARDWARE_IO_ZENITH_H

#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Hardware {

// --- INTERRUPT & TRAP ARCHITECTURE ---
struct InterruptVector {
    sigma_u64 handler_addr;
    int type; // Polling vs Vectored
};

class SovereignInterruptController : public SigmaObject {
private:
    InterruptVector m_vectors[256];
public:
    const char* type_name() const noexcept override { return "SovereignInterruptController"; }
    void RegisterHandler(int vec, sigma_u64 addr);
    void TriggerTrap(int reason); // Software-generated interrupt
};

// --- DMA & CONTROLLER LOGIC ---
class SovereignDMAController : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignDMAController"; }
    void TransferBlock(void* src, void* dest, sigma_size_t size); // No CPU intervention
};

// --- I/O SUBSYSTEM (BLOCK VS CHARACTER) ---
enum class DeviceType { BLOCK, CHARACTER, NETWORK };

class SovereignIODevice : public SigmaObject {
protected:
    DeviceType m_type;
    const char* m_name;
public:
    virtual void Read() = 0;
    virtual void Write() = 0;
};

class SovereignBlockDevice : public SovereignIODevice {
public:
    SovereignBlockDevice(const char* n) { m_type = DeviceType::BLOCK; m_name = n; }
    void Read() override; // Seek/Read/Write
    void Write() override;
};

class SovereignCharDevice : public SovereignIODevice {
public:
    SovereignCharDevice(const char* n) { m_type = DeviceType::CHARACTER; m_name = n; }
    void Read() override; // Get/Put
    void Write() override;
};

} // namespace Hardware
} // namespace SigmaOS

#endif
