# sigma-syscall-profiler

Profiles syscall usage of Linux binaries and OCI containers to prioritize
`kernel/linux_compat` implementation work.

## Usage

```bash

# Profile a binary

python3 tools/syscall_profiler/profiler.py --binary /usr/bin/nginx --output nginx.csv

# Profile an OCI container image

python3 tools/syscall_profiler/profiler.py --image nginx:latest --output nginx.csv

# Parse an existing strace log

strace -c -o strace.log nginx -g 'daemon off;'
python3 tools/syscall_profiler/profiler.py --strace-log strace.log --output nginx.csv

# JSON output

python3 tools/syscall_profiler/profiler.py --binary /usr/bin/python3 --json
```

## Output

```
Σ SigmaOS Syscall Profile ===========================
SYSCALL                         COUNT     %    SIGMA STATUS
------------------------------------------------------------
read                             4821  18.2%  🔄 partial
write                            3204  12.1%  🔄 partial
mmap                             2108   7.9%  🔄 partial
close                            1987   7.5%  🔄 partial
openat                           1654   6.2%  ❌ missing
fstat                            1432   5.4%  ❌ missing
...
Top-30 cumulative: 82.3%  Total calls: 26514
```

## CI Integration

```yaml

# .github/workflows/ci.yml

- name: Run syscall profiler
  run: |
    python3 tools/syscall_profiler/profiler.py \
      --image nginx:latest \
      --output profiles/nginx-syscalls.csv \
      --timeout 15
```

## Requirements

- Python 3.8+ (no external packages)

- `strace` for dynamic profiling (Linux only)

- `docker` for OCI image profiling (optional)
