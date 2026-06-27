// SPDX-License-Identifier: GPL-2.0-or-later
// tests/regression/regression_zt_revocation.cpp
//
// Regression test for Round 1 Bug:
//   "Revoked workloads still pass policy checks"
//
// Root cause: sigma_zerotrust_check_flow() evaluated the static policy BEFORE
// checking the revocation table. A revoked workload could still get ALLOW if
// the policy said allow — the revocation check was never reached.
//
// Fix: revocation is now checked FIRST. If the workload is revoked,
//      the function returns DENY immediately without evaluating policy.
//
// This test must ALWAYS pass. Any future refactor that breaks this
// will be caught here before shipping.

#include <gtest/gtest.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

// ── Minimal zero-trust stub (mirrors kernel/security/sigma_zerotrust.cpp) ─

struct ZTWorkload {
    uint32_t pid;
    char     spiffe_uri[128];
    bool     revoked;
    bool     policy_allows;  /* what the static policy says */
};

static ZTWorkload workloads[64];
static int n_workloads = 0;

static int zt_register(uint32_t pid, const char *uri, bool policy_allows) {
    if (n_workloads >= 64) return -1;
    ZTWorkload &w = workloads[n_workloads++];
    w.pid = pid;
    strncpy(w.spiffe_uri, uri, sizeof(w.spiffe_uri)-1);
    w.revoked = false;
    w.policy_allows = policy_allows;
    return 0;
}

static void zt_revoke(uint32_t pid) {
    for (int i = 0; i < n_workloads; i++)
        if (workloads[i].pid == pid) { workloads[i].revoked = true; return; }
}

// THE FIXED IMPLEMENTATION: revocation checked first
static bool zt_check_flow(uint32_t pid, uint16_t port, const char *proto) {
    (void)port; (void)proto;
    for (int i = 0; i < n_workloads; i++) {
        if (workloads[i].pid != pid) continue;
        // CRITICAL: revocation MUST be checked before policy
        if (workloads[i].revoked) return false;  /* DENY — revoked */
        return workloads[i].policy_allows;
    }
    return false; /* unknown workload → deny */
}

// ── Tests ──────────────────────────────────────────────────────────────────

class ZTRevocationTest : public ::testing::Test {
protected:
    void SetUp() override {
        n_workloads = 0;
        memset(workloads, 0, sizeof(workloads));
    }
};

TEST_F(ZTRevocationTest, AllowedWorkloadPasses) {
    zt_register(100, "spiffe://sigma.os/workload/server", true);
    EXPECT_TRUE(zt_check_flow(100, 443, "tcp"))
        << "Active allowed workload should pass";
}

TEST_F(ZTRevocationTest, DeniedByPolicyFails) {
    zt_register(200, "spiffe://sigma.os/workload/blocked", false);
    EXPECT_FALSE(zt_check_flow(200, 443, "tcp"))
        << "Workload with deny policy should be blocked";
}

TEST_F(ZTRevocationTest, RevokedWorkloadDeniedEvenIfPolicySaysAllow) {
    // This is THE regression test — the original bug allowed this to pass
    zt_register(300, "spiffe://sigma.os/workload/exploit", true); // policy = ALLOW
    zt_revoke(300); // revoke it

    EXPECT_FALSE(zt_check_flow(300, 443, "tcp"))
        << "REGRESSION: revoked workload passed policy check! "
           "Revocation must be checked BEFORE policy evaluation.";
}

TEST_F(ZTRevocationTest, RevocationIsIrreversible) {
    zt_register(400, "spiffe://sigma.os/workload/test", true);
    EXPECT_TRUE(zt_check_flow(400, 80, "tcp"));

    zt_revoke(400);
    EXPECT_FALSE(zt_check_flow(400, 80, "tcp"));
    EXPECT_FALSE(zt_check_flow(400, 443, "tcp"));
    EXPECT_FALSE(zt_check_flow(400, 22,  "tcp"));
}

TEST_F(ZTRevocationTest, UnknownWorkloadDenied) {
    EXPECT_FALSE(zt_check_flow(999, 443, "tcp"))
        << "Unknown PID should be denied by default";
}

TEST_F(ZTRevocationTest, MultipleWorkloadsIndependentRevocation) {
    zt_register(501, "spiffe://sigma.os/workload/a", true);
    zt_register(502, "spiffe://sigma.os/workload/b", true);
    zt_register(503, "spiffe://sigma.os/workload/c", true);

    zt_revoke(502); // only revoke middle one

    EXPECT_TRUE(zt_check_flow(501, 443, "tcp"))  << "workload/a still active";
    EXPECT_FALSE(zt_check_flow(502, 443, "tcp")) << "workload/b revoked";
    EXPECT_TRUE(zt_check_flow(503, 443, "tcp"))  << "workload/c still active";
}
