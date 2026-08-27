#!/usr/bin/env python3
import argparse
import json
import os

parser = argparse.ArgumentParser(description="Generate benchmark report")
parser.add_argument("--boot", required=True)
parser.add_argument("--memory", required=True)
parser.add_argument("--output", required=True)
args = parser.parse_args()

boot_data = {}
if os.path.exists(args.boot):
    with open(args.boot, "r") as f:
        boot_data = json.load(f)

mem_data = {}
if os.path.exists(args.memory):
    with open(args.memory, "r") as f:
        mem_data = json.load(f)

report = f"""# Performance Benchmark Report

## Boot Performance
- **Boot Time:** {boot_data.get('boot_time_ms', 'N/A')} ms
- **Kernel Init:** {boot_data.get('kernel_init_time_ms', 'N/A')} ms
- **Userland Init:** {boot_data.get('userland_init_time_ms', 'N/A')} ms

## Memory Usage
- **Peak Memory:** {mem_data.get('peak_memory_mb', 'N/A')} MB
- **Kernel Heap:** {mem_data.get('kernel_heap_mb', 'N/A')} MB
- **Userland RSS:** {mem_data.get('userland_rss_mb', 'N/A')} MB
"""

with open(args.output, "w") as f:
    f.write(report)

print(f"Report generated at {args.output}")
