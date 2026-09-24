# SigmaOS Resource Monitoring Architecture & Telemetry Specification

## 1. Overview

SigmaOS includes an integrated, zero-overhead telemetry subsystem that continuously monitors CPU performance, memory pressure, disk storage I/O, network bandwidth, GPU metrics, and thermal energy consumption.

## 2. Monitored System Subsystems

### 2.1 CPU & Compute Metrics
- **Per-Core Utilization**: User time, system/kernel time, idle time, I/O wait time, IRQ handling time.
- **Frequency Scaling**: Current core frequencies (P-cores / E-cores), governor states.
- **Pressure Stall Information (PSI)**: Quantifies compute stall duration (`some` vs `full` stall percentages over 10s, 60s, 300s windows).

### 2.2 Memory Subsystem Metrics
- **Physical Memory**: Total RAM, free RAM, available RAM, active/inactive buffer cache, anonymous pages, slab allocator pages.
- **Swap / ZRAM**: ZRAM compressed swap usage, compression ratio, swap page-ins and page-outs.
- **Kernel Samepage Merging (KSM)**: KSM page deduplication metrics (`pages_shared`, `pages_sharing`, `pages_unshared`).
- **Memory Pressure (PSI)**: PSI memory stall tracking to trigger proactive OOM intervention prior to system lockup.

### 2.3 Disk Storage & File System Metrics
- **I/O Throughput**: Read/write bytes per second across NVMe/SATA block devices.
- **I/O Operations (IOPS)**: Read/write IOPS and queue depth saturation.
- **Space Utilization (`df` / `duf`)**: Mount point capacity, inode availability, Btrfs/ZFS pool health.

### 2.4 Network Telemetry
- **Bandwidth**: Bytes transmitted and received per interface (`eth0`, `wlan0`, `wireguard0`).
- **Packet Metrics**: Packets per second, dropped packets, buffer overruns, TCP retransmissions.

```
+------------------------------------------------------------------+
|                    Sovereign Telemetry Hub                       |
|  +--------------------+  +------------------+  +---------------+ |
|  | /proc & /sys Nodes |  | eBPF Probe Engine|  | Hardware HWMON| |
|  +---------+----------+  +--------+---------+  +-------+-------+ |
+------------|----------------------|--------------------|---------+
             |                      |                    |
+------------v----------------------v--------------------v---------+
|                  Ring-Buffered Metric Storage                    |
+-----------------------------------+------------------------------+
                                    |
+-----------------------------------v------------------------------+
|             CLI Tools (btop, vmstat, iostat, duf)                |
+------------------------------------------------------------------+
```

## 3. Command-Line Telemetry Suite

- **`vmstat 1`**: Virtual memory, process, paging, block I/O, and CPU activity streaming.
- **`iostat -xz 1`**: Extended block storage device utilization and latency statistics.
- **`duf`**: Disk Usage Free overview utility with colorized filesystem graphs.
- **`netstat` / `ss`**: Active socket connections, TCP state tracking, listening ports.
