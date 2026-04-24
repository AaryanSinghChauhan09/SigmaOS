
# SigmaOS Development Workflow


This guide details how to build, run, debug, and profile the **SigmaOS Sovereign Lattice** using industrial-grade tooling.


## 🛠️ Build Automation

SigmaOS uses a recursive **Makefile** and **Taskfile.dev** for reproducible builds across 500 shards.

```bash
make all        # Build the entire lattice
make clean      # Purge build artifacts
make info       # Verify indexed suites and source count
```


## 🚀 Emulation & Running

Test the OS safely using **QEMU**.

```bash
make run
```


## 🐞 Low-Level Debugging

Debug kernel crashes and inspect registers using **GDB** and QEMU's GDB stub.

```bash
make debug
```
This launches QEMU in a paused state and attaches GDB automatically.


## 📊 Performance & Reliability

Identify memory leaks and profile shard performance using **Valgrind**.

```bash
make profile
```


## 🤖 CI/CD Integration

SigmaOS is integrated with **GitHub Actions**. Every push to `main` triggers:
1. Full lattice build.
2. Architectural verification.
3. Static analysis to ensure **Zero-Std** compliance (no forbidden `libc` calls).


## 🧪 Testing New Shards

To test a new shard:
1. Add your code to `suites/Sxxx_Name/`.
2. Run `make all`.
3. Use `sigma doctor` from the CLI to verify the shard's registration in the lattice.
