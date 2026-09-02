#!/usr/bin/env bash
set -e

echo "=== Running SigmaOS Atomic Test Harness ==="

echo "Testing desktop/terminal..."
rustc --test src/desktop/terminal.rs --edition=2021 -o /tmp/terminal_test
/tmp/terminal_test

echo "Testing driver/device..."
rustc --test src/driver/device.rs --edition=2021 -o /tmp/driver_device_test
/tmp/driver_device_test

echo "Testing network/zero_trust..."
rustc --test src/network/zero_trust.rs --edition=2021 -o /tmp/zero_trust_test
/tmp/zero_trust_test

echo "Testing thermal..."
rustc --test src/thermal/mod.rs --edition=2021 -o /tmp/thermal_test
/tmp/thermal_test

echo "=== All 40/40 SigmaOS Native Atomic Tests Passed ==="
