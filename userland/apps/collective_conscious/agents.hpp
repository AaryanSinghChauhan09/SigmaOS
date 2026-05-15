#ifndef AGENTS_HPP
#define AGENTS_HPP

#include "../../../include/libc/SovereignLibC.h"

#include "../../../include/core/SigmaOOP.hpp"

class IConsciousAgent : public SigmaOS::SigmaObject {
public:
    virtual void Pulse() = 0;
    virtual void Sync(const char* global_state) = 0;
};

class SecurityConsciousAgent : public IConsciousAgent {
public:
    const char* type_name() const noexcept override { return "SecurityConsciousAgent"; }
    void Pulse() override {
        sigma_printf("[COLLECTIVE/SECURITY]: Auditing perimeter for neural anomalies...\n");
    }
    void Sync(const char* state) override {
        sigma_printf("[COLLECTIVE/SECURITY]: Received state: %s. Adjusting firewall entropy.\n", state);
    }
};

class ResourceConsciousAgent : public IConsciousAgent {
public:
    const char* type_name() const noexcept override { return "ResourceConsciousAgent"; }
    void Pulse() override {
        sigma_printf("[COLLECTIVE/RESOURCE]: Optimizing silicon-power distribution across mesh.\n");
    }
    void Sync(const char* state) override {
        sigma_printf("[COLLECTIVE/RESOURCE]: Global state '%s' recognized. Reallocating VMM pages.\n", state);
    }
};

#endif
