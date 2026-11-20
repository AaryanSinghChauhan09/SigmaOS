#!/usr/bin/env bash
# SigmaOS Sovereign V35 Supremacy Benchmark: Arch Linux & Omarchy Integration Test
# Validates Pacman v7 ALPM translation, Wayland Hyprland compositor orchestration,
# Wallust dynamic themes, and Port-emerge USE flag resolution.
set -euo pipefail

echo "========================================================================"
echo "    SIGMAOS SOVEREIGN V35 APEX BENCHMARK (ARCH & OMARCHY MASTERY)       "
echo "========================================================================"

echo "[1/4] Arch Linux Pacman & ALPM Rolling Engine:"
cargo test --manifest-path=/home/aaryansinghchauhan/SigmaOS-arch/Cargo.toml --quiet
echo "  [✔] Passed: Pacman -Syu rolling upgrade and AUR RPC resolvers verified"

echo "[2/4] Omarchy Wayland & Hyprland Dynamic Compositor Engine:"
cargo test --manifest-path=/home/aaryansinghchauhan/SigmaOS-omarchy/Cargo.toml --quiet
echo "  [✔] Passed: Hyprland layout generation, workspace rules & doctor diagnostics verified"

echo "[3/4] Gentoo Portage & USE-Flag Optimization Engine:"
cargo test --manifest-path=/home/aaryansinghchauhan/SigmaOS-linux-gentoo/Cargo.toml --quiet
echo "  [✔] Passed: Profile make.conf synthesis & conditional USE dependency resolution verified"

echo "[4/4] Debian Substrate Compatibility & DPKG Control Stanzas:"
cargo test --manifest-path=/home/aaryansinghchauhan/SigmaOS-linux-debian/Cargo.toml --quiet
echo "  [✔] Passed: Debian control file generation and dpkg query verification passed"

echo "========================================================================"
echo "    ALL 4 V35 INTEGRATION BENCHMARKS CONFIRMED: FULL OS READINESS       "
echo "========================================================================"
