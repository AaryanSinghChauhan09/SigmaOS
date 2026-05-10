#include <gtest/gtest.h>
#include "security/sigma_sandbox.h"

class SovereignSandboxTest : public ::testing::Test {
protected:
    void SetUp() override {
        // Initialize sandbox with default policy
        sandbox_init();
    }
};

TEST_F(SovereignSandboxTest, BlockedSyscallReturnsError) {
    // Attempting a syscall that is NOT in the global allowed list (e.g., sigma_raw_disk_write)
    // Assuming 0x99 is a forbidden syscall ID
    bool allowed = sandbox_check_syscall(0x99);
    EXPECT_FALSE(allowed);
}

TEST_F(SovereignSandboxTest, AllowedSyscallReturnsSuccess) {
    // sigma_yield is allowed in the global policy (ID 0x01)
    bool allowed = sandbox_check_syscall(0x01);
    EXPECT_TRUE(allowed);
}

TEST_F(SovereignSandboxTest, CapabilityEscalationDenied) {
    // Attempting to access eBPF without the capability
    bool has_cap = sandbox_has_capability("SovereignWASM", "EBPF_INJECT");
    EXPECT_FALSE(has_cap);
}

TEST_F(SovereignSandboxTest, ShardSpecificCapabilityAllowed) {
    // SovereignMonitor HAS eBPF capability
    bool has_cap = sandbox_has_capability("SovereignMonitor", "EBPF_INJECT");
    EXPECT_TRUE(has_cap);
}
