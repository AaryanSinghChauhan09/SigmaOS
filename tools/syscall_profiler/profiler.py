#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# tools/syscall_profiler/profiler.py
# Profiles syscall usage of a binary or container image.
# Outputs a CSV to help prioritize linux_compat syscall implementation.
#
# Usage:
#   python3 profiler.py --binary /usr/bin/nginx --output nginx_syscalls.csv
#   python3 profiler.py --image nginx:latest --output nginx_syscalls.csv
#   python3 profiler.py --strace-log strace.log --output syscalls.csv
#
# No external dependencies. Uses strace (Linux) or manual log parsing.

import sys
import os
import subprocess
import csv
import json
import tempfile
import time
import argparse
import shutil
from collections import Counter

# ── All known x86_64 Linux syscalls (hand-coded, no external libs) ─────────
SYSCALL_NAMES = {
    0: "read", 1: "write", 2: "open", 3: "close", 4: "stat",
    5: "fstat", 6: "lstat", 7: "poll", 8: "lseek", 9: "mmap",
    10: "mprotect", 11: "munmap", 12: "brk", 13: "rt_sigaction",
    14: "rt_sigprocmask", 15: "rt_sigreturn", 16: "ioctl",
    17: "pread64", 18: "pwrite64", 19: "readv", 20: "writev",
    21: "access", 22: "pipe", 23: "select", 24: "sched_yield",
    25: "mremap", 26: "msync", 27: "mincore", 28: "madvise",
    32: "dup", 33: "dup2", 35: "nanosleep", 39: "getpid",
    41: "socket", 42: "connect", 43: "accept", 44: "sendto",
    45: "recvfrom", 46: "sendmsg", 47: "recvmsg", 48: "shutdown",
    49: "bind", 50: "listen", 51: "getsockname", 52: "getpeername",
    54: "setsockopt", 55: "getsockopt", 56: "clone", 57: "fork",
    58: "vfork", 59: "execve", 60: "exit", 61: "wait4", 62: "kill",
    63: "uname", 72: "fcntl", 73: "flock", 74: "fsync", 75: "fdatasync",
    76: "truncate", 77: "ftruncate", 78: "getdents", 79: "getcwd",
    80: "chdir", 81: "fchdir", 82: "rename", 83: "mkdir", 84: "rmdir",
    85: "creat", 86: "link", 87: "unlink", 88: "symlink", 89: "readlink",
    90: "chmod", 91: "fchmod", 92: "chown", 93: "fchown", 94: "lchown",
    95: "umask", 96: "gettimeofday", 97: "getrlimit", 102: "getuid",
    104: "getgid", 107: "geteuid", 108: "getegid", 110: "getppid",
    202: "futex", 218: "set_tid_address", 228: "clock_gettime",
    229: "clock_getres", 230: "clock_nanosleep",
    231: "exit_group", 232: "epoll_wait", 233: "epoll_ctl",
    257: "openat", 262: "newfstatat", 263: "unlinkat",
    264: "renameat", 265: "linkat", 266: "symlinkat",
    291: "epoll_create1", 293: "pipe2", 295: "openat",
    318: "getrandom", 319: "memfd_create", 322: "execveat",
    332: "statx", 334: "rseq",
}

class SyscallProfiler:
    def __init__(self):
        self.counts = Counter()

    def profile_binary(self, binary_path, args=None, timeout=10):
        """Profile syscalls of a binary using strace."""
        if not shutil.which("strace"):
            print("[profiler] strace not found — using static analysis mode")
            return self._analyze_elf(binary_path)

        strace_out = tempfile.NamedTemporaryFile(suffix=".strace", delete=False)
        strace_path = strace_out.name
        strace_out.close()

        cmd = ["strace", "-c", "-o", strace_path, "-e", "trace=all", binary_path]
        if args:
            cmd.extend(args)

        try:
            subprocess.run(cmd, timeout=timeout, capture_output=True)
        except subprocess.TimeoutExpired:
            pass
        except Exception as e:
            print(f"[profiler] strace failed: {e}")

        if os.path.exists(strace_path):
            self._parse_strace_summary(strace_path)
            os.unlink(strace_path)

        return self.counts

    def profile_image(self, image_name, cmd="/bin/sh -c 'sleep 2'", timeout=30):
        """Profile syscalls inside a Docker/OCI container."""
        if not shutil.which("docker"):
            print("[profiler] docker not found — cannot profile OCI image")
            return self.counts

        strace_cmd = f"strace -ff -c -o /tmp/strace {cmd}"
        docker_cmd = [
            "docker", "run", "--rm",
            "--cap-add=SYS_PTRACE",
            "--security-opt=seccomp:unconfined",
            image_name,
            "/bin/sh", "-c", strace_cmd + "; cat /tmp/strace* 2>/dev/null"
        ]

        try:
            result = subprocess.run(
                docker_cmd, timeout=timeout,
                capture_output=True, text=True
            )
            self._parse_strace_output_lines(result.stdout.splitlines())
        except Exception as e:
            print(f"[profiler] docker run failed: {e}")

        return self.counts

    def parse_strace_log(self, log_path):
        """Parse an existing strace -c output file."""
        if not os.path.exists(log_path):
            print(f"[profiler] log file not found: {log_path}")
            return self.counts
        with open(log_path, "r", errors="replace") as f:
            lines = f.readlines()
        self._parse_strace_output_lines(lines)
        return self.counts

    def _parse_strace_summary(self, path):
        """Parse strace -c summary format."""
        try:
            with open(path, "r", errors="replace") as f:
                lines = f.readlines()
            self._parse_strace_output_lines(lines)
        except Exception:
            pass

    def _parse_strace_output_lines(self, lines):
        """Parse strace output lines — both -c summary and raw formats."""
        for line in lines:
            line = line.strip()
            # strace -c summary: "  0.01  1234 5678  9  12 write"
            parts = line.split()
            if len(parts) >= 6 and parts[0].replace(".", "").isdigit():
                try:
                    count = int(parts[3])
                    syscall_name = parts[-1]
                    if syscall_name not in ("syscall", "calls", "errors"):
                        self.counts[syscall_name] += count
                except (ValueError, IndexError):
                    pass
            # Raw strace line: "write(1, "hello", 5) = 5"
            elif "(" in line and not line.startswith("%"):
                syscall = line.split("(")[0].strip().lstrip("+-0123456789 \t")
                if syscall and syscall.isidentifier():
                    self.counts[syscall] += 1

    def _analyze_elf(self, binary_path):
        """Static ELF analysis — look for syscall numbers in .rodata."""
        try:
            with open(binary_path, "rb") as f:
                data = f.read()
            # Look for syscall instruction sequences (0F 05 on x86_64)
            # and the mov eax, <imm32> before them
            count = data.count(b"\x0f\x05")
            self.counts["(elf-syscall-sites)"] = count
        except Exception:
            pass
        return self.counts

    def write_csv(self, output_path, total_comment=None):
        """Write results to CSV."""
        total = sum(self.counts.values())
        with open(output_path, "w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow(["syscall", "count", "percent", "sigma_status"])
            for syscall, count in self.counts.most_common():
                pct = round(count * 100 / total, 2) if total > 0 else 0
                status = self._sigma_status(syscall)
                writer.writerow([syscall, count, pct, status])
        print(f"[profiler] Written: {output_path} ({len(self.counts)} syscalls, {total} calls)")

    def _sigma_status(self, name):
        """Return SigmaOS implementation status for a syscall."""
        implemented = {
            "getpid", "getppid", "getuid", "geteuid", "exit", "exit_group",
            "nanosleep", "clock_gettime", "uname", "getrandom",
            "set_tid_address", "mmap", "munmap", "brk", "futex",
        }
        partial = {
            "read", "write", "open", "close", "mprotect",
            "socket", "connect", "bind", "listen", "accept",
        }
        if name in implemented:
            return "implemented"
        elif name in partial:
            return "partial"
        else:
            return "missing"

    def print_report(self, top_n=30):
        """Print a human-readable report."""
        total = sum(self.counts.values())
        print(f"\n{'Σ SigmaOS Syscall Profile':=<60}")
        print(f"{'SYSCALL':<30} {'COUNT':>8} {'%':>6}  {'SIGMA STATUS'}")
        print("-" * 60)
        cumulative = 0
        for i, (syscall, count) in enumerate(self.counts.most_common(top_n)):
            pct = count * 100 / total if total > 0 else 0
            cumulative += pct
            status = self._sigma_status(syscall)
            marker = "✅" if status == "implemented" else ("🔄" if status == "partial" else "❌")
            print(f"{syscall:<30} {count:>8} {pct:>5.1f}%  {marker} {status}")
        print("-" * 60)
        print(f"Top-{top_n} cumulative: {cumulative:.1f}%  Total calls: {total}")
        missing = [s for s, _ in self.counts.most_common(50) if self._sigma_status(s) == "missing"]
        if missing:
            print(f"\nTop missing syscalls to implement next:")
            for s in missing[:10]:
                print(f"  ❌ {s}")


def main():
    parser = argparse.ArgumentParser(
        description="SigmaOS Syscall Profiler — prioritize linux_compat work"
    )
    parser.add_argument("--binary", help="Path to ELF binary to profile")
    parser.add_argument("--image", help="OCI image name (requires docker)")
    parser.add_argument("--strace-log", help="Path to existing strace -c log")
    parser.add_argument("--output", default="syscalls.csv", help="Output CSV path")
    parser.add_argument("--top", type=int, default=30, help="Top N syscalls to display")
    parser.add_argument("--timeout", type=int, default=10, help="Profiling timeout (seconds)")
    parser.add_argument("--json", action="store_true", help="Output JSON instead of CSV")
    args = parser.parse_args()

    profiler = SyscallProfiler()

    if args.strace_log:
        profiler.parse_strace_log(args.strace_log)
    elif args.binary:
        profiler.profile_binary(args.binary, timeout=args.timeout)
    elif args.image:
        profiler.profile_image(args.image, timeout=args.timeout)
    else:
        print("[profiler] No input specified. Use --binary, --image, or --strace-log")
        parser.print_help()
        sys.exit(1)

    if not profiler.counts:
        print("[profiler] No syscalls captured. Try with a different binary or increase --timeout")
        sys.exit(1)

    profiler.print_report(top_n=args.top)

    if args.json:
        with open(args.output.replace(".csv", ".json"), "w") as f:
            json.dump(dict(profiler.counts.most_common()), f, indent=2)
    else:
        profiler.write_csv(args.output)


if __name__ == "__main__":
    main()
