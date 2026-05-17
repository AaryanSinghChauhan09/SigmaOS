# Zero-Dependency Architecture

## Principle
All critical kernel paths in SigmaOS run without reliance on:
- High-level language runtimes (Python, JS, Go)
- Standard C/C++ libraries (stdlib, libc, stdio, string)
- Pre-defined OS allocators (`malloc`, `free`)

## Custom Primitives
| stdlib symbol | SigmaOS replacement |
|---|---|
| `malloc`    | `sigma_malloc`    |
| `free`      | `sigma_free`      |
| `memcpy`    | `sigma_memcpy`    |
| `memset`    | `sigma_memset`    |
| `strlen`    | `sigma_strlen`    |
| `printf`    | `sigma_log_raw`   |
| `std::string` | `const char*` |
| `std::vector` | `SigmaVector` |

## Enforcement
All kernel `.cpp`/`.h` files are automatically scanned
by `omega_fix.py` to purge stdlib symbols on every sync.
