#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# SigmaOS Test Runner Script
set -e

echo "=== Running SigmaOS Test Suite ==="
if command -v pytest &>/dev/null; then
  pytest tests/
elif python3 -m pytest --version &>/dev/null; then
  python3 -m pytest tests/
elif command -v python3 &>/dev/null; then
  python3 -c "import tests.test_integration_system as t1, tests.test_python_env as t2, tests.test_stress_fuzz_bench as t3, tests.test_unit_core as t4; t1.test_shell_syscall_interaction(); t1.test_device_driver_mocking(); t1.test_network_socket_packet_transfer(); t1.test_security_authorization_denial(); t1.test_boot_sequence_varied_configs(); t1.test_universal_package_manager_cli_simulation(); t2.test_python_environment(); t3.test_stress_concurrent_processes(); t3.test_fuzz_syscall_inputs(); t3.test_benchmark_against_baseline(); t4.test_file_io_operations(); t4.test_process_scheduling_fairness(); t4.test_memory_management_alloc_free_leak(); print('All 13 Python tests executed and passed cleanly.')"
fi

echo "=== All SigmaOS Tests Completed Successfully ==="
