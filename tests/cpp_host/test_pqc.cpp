#include "include/sigma_types.h"
#define SIGMA_HOST
#include <gtest/gtest.h>
#include "security/sigma_pqc.h"
#include "../../include/core/sigma_kernel_types.h"

// Stubs for hardware/kernel functions not available on host
extern "C" {
    void log_emit(sigma_u32 severity, const char* message) { (void)severity; (void)message; }
    void log_emit_f(sigma_u32 severity, const char* format, ...) { (void)severity; (void)format; }
    void* SIGMA_NULL = nullptr;
    
    // Implementation for the declaration in sigma_kernel_types.h
    sigma_u64 cpu_rdtsc(void) { return 123456789ULL; }
}

namespace SigmaOS {
namespace Kernel {
namespace Security {

class PQCTest : public ::testing::Test {
protected:
    void SetUp() override {
        SigmaOS::Kernel::Security::SovereignPQCEngine::getInstance().init();
    }
};

TEST_F(PQCTest, SingletonInstance) {
    auto& instance1 = SigmaOS::Kernel::Security::SovereignPQCEngine::getInstance();
    auto& instance2 = SigmaOS::Kernel::Security::SovereignPQCEngine::getInstance();
    EXPECT_EQ(&instance1, &instance2);
}

TEST_F(PQCTest, ShardSigning) {
    sigma_u8 signature[64];
    SigmaOS::Kernel::Security::SovereignPQCEngine::getInstance().signShard(42, signature);
    
    // Verify signature is generated (not all zeros)
    bool all_zeros = true;
    for(int i=0; i<64; i++) if(signature[i] != 0) all_zeros = false;
    EXPECT_FALSE(all_zeros);
}

TEST_F(PQCTest, ShardVerification) {
    sigma_u8 signature[64];
    SigmaOS::Kernel::Security::SovereignPQCEngine::getInstance().signShard(42, signature);
    
    bool valid = SigmaOS::Kernel::Security::SovereignPQCEngine::getInstance().verifyShard(42, signature);
    EXPECT_TRUE(valid);
}

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

extern "C" {


} // extern "C"
