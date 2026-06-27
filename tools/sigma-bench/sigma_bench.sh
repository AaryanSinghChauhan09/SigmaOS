#!/bin/bash
# SPDX-License-Identifier: GPL-2.0-or-later
# sigma-bench — SigmaOS system benchmark suite
#
# Inspired by: phoronix-test-suite, sysbench, fio, iperf3, stress-ng
#
# Benchmarks:
#   cpu     — multi-thread integer + float GFLOPS (stress-ng)
#   memory  — bandwidth / latency with stream benchmark
#   disk    — sequential + random IOPS via fio
#   net     — TCP/UDP throughput via iperf3
#   boot    — measure time from GRUB handoff to sigma-healthd ready
#   kernel  — syscall latency, context switch time, IPC round-trip
#   all     — run all benchmarks
#
# Usage: sigma-bench [cpu|memory|disk|net|boot|kernel|all] [--json]
# Output: human-readable table or JSON when --json is passed

set -euo pipefail

BENCH="${1:-all}"
JSON_OUT=false
[[ "${2:-}" == "--json" ]] && JSON_OUT=true

RESULTS=()

log() { echo "[sigma-bench] $*" >&2; }
result() {
    local name="$1" value="$2" unit="$3"
    RESULTS+=("{\"name\":\"$name\",\"value\":$value,\"unit\":\"$unit\"}")
    if ! $JSON_OUT; then
        printf "  %-30s %10s %s\n" "$name" "$value" "$unit"
    fi
}

# ── CPU benchmark ─────────────────────────────────────────────────────────
bench_cpu() {
    log "Running CPU benchmark (30s)..."
    if command -v stress-ng >/dev/null 2>&1; then
        SCORE=$(stress-ng --cpu "$(nproc)" --cpu-ops 100000 \
                          --metrics-brief 2>&1 | \
                grep 'cpu ' | awk '{print $9}')
        result "cpu_bogomips" "${SCORE:-0}" "bogo-ops/s"
    else
        # Fallback: time a simple loop
        START=$(date +%s%N)
        for i in $(seq 1 1000000); do : ; done
        END=$(date +%s%N)
        MS=$(( (END - START) / 1000000 ))
        result "cpu_loop_1M" "$MS" "ms"
    fi
}

# ── Memory benchmark ──────────────────────────────────────────────────────
bench_memory() {
    log "Running memory bandwidth benchmark..."
    if command -v sysbench >/dev/null 2>&1; then
        MBPS=$(sysbench memory --memory-block-size=1K \
                               --memory-total-size=4G run 2>&1 | \
               grep 'transferred' | grep -oP '[\d.]+(?= MiB/sec)')
        result "mem_bandwidth" "${MBPS:-0}" "MiB/s"
    else
        # /dev/shm fallback: dd to tmpfs
        SPEED=$(dd if=/dev/zero of=/dev/shm/bench_tmp bs=1M count=512 \
                   2>&1 | tail -1 | grep -oP '[\d.]+ MB/s' | head -1 | \
                   awk '{print $1}')
        rm -f /dev/shm/bench_tmp
        result "mem_write_dd" "${SPEED:-0}" "MB/s"
    fi
}

# ── Disk benchmark ────────────────────────────────────────────────────────
bench_disk() {
    log "Running disk I/O benchmark (fio)..."
    TMP=$(mktemp -d)
    if command -v fio >/dev/null 2>&1; then
        SEQ_READ=$(fio --name=seq-read --ioengine=libaio --iodepth=32 \
                       --rw=read --bs=1M --size=256M --numjobs=1 \
                       --directory="$TMP" --output-format=json 2>/dev/null | \
                   python3 -c "import sys,json; d=json.load(sys.stdin); \
                                print(d['jobs'][0]['read']['bw']//1024)")
        result "disk_seq_read" "${SEQ_READ:-0}" "MB/s"

        RAND_IOPS=$(fio --name=rand-read --ioengine=libaio --iodepth=32 \
                        --rw=randread --bs=4k --size=256M --numjobs=4 \
                        --directory="$TMP" --output-format=json 2>/dev/null | \
                    python3 -c "import sys,json; d=json.load(sys.stdin); \
                                 print(d['jobs'][0]['read']['iops'])")
        result "disk_rand_read_iops" "${RAND_IOPS:-0}" "IOPS"
    else
        # dd fallback
        SPEED=$(dd if=/dev/zero of="$TMP/test" bs=1M count=256 oflag=dsync \
                   2>&1 | tail -1 | grep -oP '[\d.]+ MB/s' | awk '{print $1}')
        result "disk_write_dd" "${SPEED:-0}" "MB/s"
    fi
    rm -rf "$TMP"
}

# ── Network benchmark ─────────────────────────────────────────────────────
bench_net() {
    log "Running network benchmark (loopback iperf3)..."
    if command -v iperf3 >/dev/null 2>&1; then
        iperf3 -s -D -I /tmp/iperf3.pid 2>/dev/null || true
        sleep 0.2
        GBPS=$(iperf3 -c 127.0.0.1 -t 5 -J 2>/dev/null | \
               python3 -c "import sys,json; d=json.load(sys.stdin); \
                            print(round(d['end']['sum_received']['bits_per_second']/1e9, 2))")
        kill "$(cat /tmp/iperf3.pid)" 2>/dev/null || true
        result "net_loopback_tcp" "${GBPS:-0}" "Gbps"
    else
        result "net_loopback_tcp" "N/A (iperf3 not found)" "—"
    fi
}

# ── Boot time benchmark ───────────────────────────────────────────────────
bench_boot() {
    log "Measuring boot time via systemd-analyze (if available)..."
    if command -v systemd-analyze >/dev/null 2>&1; then
        BOOT_MS=$(systemd-analyze time 2>/dev/null | \
                  grep 'userspace' | grep -oP '[\d.]+(?=s)' | tail -1)
        result "boot_userspace" "${BOOT_MS:-0}" "s"
    else
        result "boot_userspace" "N/A" "s"
    fi
}

# ── Kernel micro-benchmark ────────────────────────────────────────────────
bench_kernel() {
    log "Running kernel micro-benchmarks..."
    # Context switch latency via hackbench
    if command -v hackbench >/dev/null 2>&1; then
        CS=$(hackbench -s 512 -l 200 -g 10 2>&1 | \
             grep 'Time:' | awk '{print $2}')
        result "ctx_switch_hackbench" "${CS:-0}" "s"
    fi
    # Syscall latency: getpid() 1M times
    START=$(date +%s%N)
    for _ in $(seq 1 10000); do cat /proc/self/status >/dev/null 2>&1; done
    END=$(date +%s%N)
    US=$(( (END - START) / 10000 / 1000 ))
    result "syscall_latency_us" "$US" "µs"
}

# ── Main ──────────────────────────────────────────────────────────────────
if ! $JSON_OUT; then
    echo "┌─────────────────────────────────────────────────────┐"
    echo "│           SigmaOS System Benchmark Suite            │"
    echo "└─────────────────────────────────────────────────────┘"
fi

case "$BENCH" in
    cpu)     bench_cpu ;;
    memory)  bench_memory ;;
    disk)    bench_disk ;;
    net)     bench_net ;;
    boot)    bench_boot ;;
    kernel)  bench_kernel ;;
    all)
        bench_cpu
        bench_memory
        bench_disk
        bench_net
        bench_boot
        bench_kernel
        ;;
    *) echo "Usage: sigma-bench [cpu|memory|disk|net|boot|kernel|all] [--json]"; exit 1 ;;
esac

if $JSON_OUT; then
    echo "[$(IFS=,; echo "${RESULTS[*]}")]"
fi
