#include <gtest/gtest.h>
#include "../../include/sigma_pqc.h"
#include "../../include/sigma_kernel_types.h"

// Stubs for hardware/kernel functions not available on host
extern "C" {
    void log_emit(sigma_u32 severity, const char* message) { (void)severity; (void)message; }
    void log_emit_f(sigma_u32 severity, const char* format, ...) { (void)severity; (void)format; }
    sigma_u64 cpu_rdtsc() { return 123456789ULL; }
    void* SIGMA_NULL = nullptr;
}

namespace SigmaOS {
namespace Kernel {
namespace Security {

class PQCTest : public ::testing::Test {
protected:
    void SetUp() override {
        SovereignPQCEngine::getInstance().init();
    }
};

TEST_F(PQCTest, SingletonInstance) {
    auto& instance1 = SovereignPQCEngine::getInstance();
    auto& instance2 = SovereignPQCEngine::getInstance();
    EXPECT_EQ(&instance1, &instance2);
}

TEST_F(PQCTest, ShardSigning) {
    sigma_u8 signature[64];
    SovereignPQCEngine::getInstance().signShard(42, signature);
    
    // Verify signature is generated (not all zeros)
    bool all_zeros = true;
    for(int i=0; i<64; i++) if(signature[i] != 0) all_zeros = false;
    EXPECT_FALSE(all_zeros);
}

TEST_F(PQCTest, ShardVerification) {
    sigma_u8 signature[64];
    SovereignPQCEngine::getInstance().signShard(42, signature);
    
    bool valid = SovereignPQCEngine::getInstance().verifyShard(42, signature);
    EXPECT_TRUE(valid);
}

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS
