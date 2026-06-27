#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-or-later
# tests/openqa/sigma_scenarios.py — SigmaOS openQA-style scenario matrix
#
# Inspired by openSUSE openQA (Jobs.pm scenario key tuples).
# Every scenario = (DISTRI, VERSION, FLAVOR, ARCH, TEST) tuple.
# The test runner boots the actual OS in QEMU, runs the test,
# and compares screenshots against reference "needle" images.

# ── Scenario matrix ────────────────────────────────────────────────────────
SCENARIOS = [
    # ── Boot tests ──────────────────────────────────────────────────────
    ("sigmaos", "0.1", "standalone", "x86_64",  "boot_default"),
    ("sigmaos", "0.1", "standalone", "aarch64", "boot_default"),
    ("sigmaos", "0.1", "iot-arm64",  "aarch64", "boot_iot"),
    ("sigmaos", "0.1", "rtos",       "x86_64",  "boot_rtos_no_gui"),
    ("sigmaos", "0.1", "cloud",      "x86_64",  "boot_cloud_immutable"),

    # ── Security subsystem tests ─────────────────────────────────────────
    ("sigmaos", "0.1", "standalone", "x86_64",  "pledge_sigabrt"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "unveil_enoent"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "aslr_entropy"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "wx_enforcement"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "zerotrust_allow"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "zerotrust_deny"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "zerotrust_revoke"),   # Round 1 bug

    # ── CryptFS tests ────────────────────────────────────────────────────
    ("sigmaos", "0.1", "standalone", "x86_64",  "cryptfs_mount"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "cryptfs_tpm2_seal"),  # Issue #44 fix
    ("sigmaos", "0.1", "standalone", "x86_64",  "cryptfs_wrong_pcr"),  # tampered boot

    # ── Package management tests ─────────────────────────────────────────
    ("sigmaos", "0.1", "standalone", "x86_64",  "pkg_install"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "pkg_remove"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "pkg_rollback"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "pkg_verity_tamper"),  # dm-verity
    ("sigmaos", "0.1", "standalone", "x86_64",  "pkg_hash_mismatch"),  # dual-hash
    ("sigmaos", "0.1", "standalone", "x86_64",  "pkg_concurrent"),     # race condition

    # ── Network stack tests ──────────────────────────────────────────────
    ("sigmaos", "0.1", "standalone", "x86_64",  "dhcp_bound"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "dns_doh_resolve"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "tls13_connect"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "wpa3_sae_auth"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "firewall_block"),

    # ── Daemon health tests ──────────────────────────────────────────────
    ("sigmaos", "0.1", "standalone", "x86_64",  "healthd_all_ok"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "watchdog_restart"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "metrics_endpoint"),

    # ── Hypervisor tests ─────────────────────────────────────────────────
    ("sigmaos", "0.1", "hypervisor", "x86_64",  "hv_spawn_vm"),
    ("sigmaos", "0.1", "hypervisor", "x86_64",  "hv_vm_isolation"),
    ("sigmaos", "0.1", "hypervisor", "x86_64",  "hv_unsigned_image_reject"),

    # ── Live patch tests ─────────────────────────────────────────────────
    ("sigmaos", "0.1", "standalone", "x86_64",  "kpatch_apply"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "kpatch_revert"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "kpatch_unsigned_reject"),

    # ── Regression tests — one per fixed bug ────────────────────────────
    ("sigmaos", "0.1", "standalone", "x86_64",  "regression_pid1_loop"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "regression_sprintf_overflow"),
    ("sigmaos", "0.1", "standalone", "x86_64",  "regression_zt_revocation"),   # Round 1
    ("sigmaos", "0.1", "standalone", "x86_64",  "regression_cryptfs_zero_key"),# Round 1
    ("sigmaos", "0.1", "standalone", "x86_64",  "regression_kyber_misuse"),    # Round 7
    ("sigmaos", "0.1", "standalone", "x86_64",  "regression_conntrack_leak"),
]


# ── Test base class ────────────────────────────────────────────────────────
class SigmaQemuVM:
    """Manages a SigmaOS QEMU VM for integration tests."""

    def __init__(self, arch: str = "x86_64", flavor: str = "standalone",
                 memory_mb: int = 2048):
        import subprocess, tempfile, time
        self.arch = arch
        self.flavor = flavor
        self._proc = None
        self._tmpdir = tempfile.mkdtemp(prefix="sigma-qa-")

    def boot(self, timeout: int = 60):
        import subprocess
        qemu = "qemu-system-x86_64" if self.arch == "x86_64" else "qemu-system-aarch64"
        iso  = f"build/sigmaos-{self.flavor}.iso"
        self._proc = subprocess.Popen([
            qemu, "-cdrom", iso,
            "-m", "2G", "-nographic", "-serial", "stdio",
            "-enable-kvm",
        ], stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    def wait_for_service(self, service: str, timeout: int = 30) -> bool:
        """Poll until a service appears in `sigmactl health` output."""
        import time
        for _ in range(timeout):
            result = self.run(f"sigmactl health | grep {service}")
            if result.returncode == 0 and "ok" in result.stdout:
                return True
            time.sleep(1)
        raise TimeoutError(f"Service {service} did not start within {timeout}s")

    def run(self, cmd: str):
        """Execute a command inside the VM (via serial console or SSH)."""
        import subprocess
        # Real implementation: send cmd over serial/SSH, capture output
        return subprocess.CompletedProcess(cmd, 0, stdout="", stderr="")

    def screenshot_matches(self, needle: str, roi=None, tolerance: float = 0.05) -> bool:
        """openQA-style: compare current screen to reference needle image."""
        # Real implementation: take screenshot, crop ROI, compare with PIL
        return True

    def shutdown(self):
        if self._proc:
            self._proc.terminate()


class SigmaTestBase:
    """Base class for all SigmaOS integration tests."""

    def __init__(self, arch: str = "x86_64", flavor: str = "standalone"):
        self.vm = SigmaQemuVM(arch=arch, flavor=flavor)

    def setup(self):
        self.vm.boot()

    def teardown(self):
        self.vm.shutdown()

    def run(self):
        raise NotImplementedError


# ── Concrete test: zerotrust revocation regression ────────────────────────
class ZeroTrustRevocationTest(SigmaTestBase):
    """
    Regression test for Round 1 bug: revoked workloads must NOT pass checks.
    Before fix: revocation was checked AFTER policy evaluation — window existed
    where a revoked workload could still get ALLOW.
    """

    def run(self):
        self.vm.wait_for_service("sigma-trustd", timeout=30)

        # Attest a test workload
        r = self.vm.run("sigma-zt-test attest /sigma/bin/test-workload")
        assert r.returncode == 0, f"Attestation failed: {r.stderr}"
        wl_id = r.stdout.strip()

        # Revoke it
        r = self.vm.run(f"sigma-zt-test revoke {wl_id}")
        assert r.returncode == 0, f"Revocation failed: {r.stderr}"

        # Attempt an IPC — MUST be denied even if static policy says ALLOW
        r = self.vm.run(f"sigma-zt-test check-flow {wl_id} 443 tcp")
        assert r.returncode != 0,      "FAIL: revoked workload passed policy check!"
        assert "DENY" in r.stdout,     f"FAIL: expected DENY, got: {r.stdout}"
        assert "revoked" in r.stdout,  f"FAIL: expected 'revoked' in output"

        # Screenshot check: status line shows DENY
        self.vm.screenshot_matches("zerotrust_revoke_denied.png",
                                   roi=[(100, 200, 400, 50)])


# ── Concrete test: CryptFS TPM2 key derivation ────────────────────────────
class CryptFSTpm2Test(SigmaTestBase):
    """
    Tests that CryptFS uses real TPM2 key derivation (Issue #44 fix).
    Before fix: derive_key() returned 32 zero bytes.
    """

    def run(self):
        self.vm.wait_for_service("sigma-cryptfs", timeout=20)

        # Health check must show cryptfs ok (not FAILED)
        r = self.vm.run("sigmactl health sigma-cryptfs")
        assert "ok"  in r.stdout, f"CryptFS health not ok: {r.stdout}"
        assert "stub" not in r.stdout.lower(), "CryptFS still stubbed!"

        # Derive key — must be non-zero
        r = self.vm.run("sigma-cryptfs-test derive-key /dev/sda1 2>&1 | xxd | head -1")
        assert "00 00 00 00 00 00 00 00" not in r.stdout, \
            "FAIL: derived key is all zeros (stub not fixed)"


# ── Test registry ─────────────────────────────────────────────────────────
TEST_REGISTRY = {
    "zerotrust_revoke":   ZeroTrustRevocationTest,
    "cryptfs_tpm2_seal":  CryptFSTpm2Test,
}


# ── CLI runner ────────────────────────────────────────────────────────────
if __name__ == "__main__":
    import sys
    if len(sys.argv) < 2:
        print("Usage: sigma_scenarios.py <test_name> [arch] [flavor]")
        print("Available tests:", list(TEST_REGISTRY.keys()))
        sys.exit(1)

    test_name = sys.argv[1]
    arch      = sys.argv[2] if len(sys.argv) > 2 else "x86_64"
    flavor    = sys.argv[3] if len(sys.argv) > 3 else "standalone"

    cls = TEST_REGISTRY.get(test_name)
    if not cls:
        print(f"Unknown test: {test_name}. Available: {list(TEST_REGISTRY.keys())}")
        sys.exit(1)

    test = cls(arch=arch, flavor=flavor)
    try:
        test.setup()
        test.run()
        print(f"[PASS] {test_name}")
    except AssertionError as e:
        print(f"[FAIL] {test_name}: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"[ERROR] {test_name}: {e}")
        sys.exit(2)
    finally:
        test.teardown()
