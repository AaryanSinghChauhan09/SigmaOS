# Troubleshooting SigmaOS

## Build Issues

### "error[E0433]: cannot find type X"

Missing type definitions from unmerged branches.

**Fix:** Ensure you have the latest `main` branch:
```bash
git fetch origin
git checkout main
git pull origin main
```

### "error[E0119]: conflicting implementations"

Duplicate `impl` blocks for the same type.

**Fix:** Find and remove duplicates:
```bash
cargo +nightly check 2>&1 | grep "E0119" | head -20
```
Then manually remove the duplicate `impl` block in the indicated file.

### "error[E0252]: name defined multiple times"

Usually from duplicate `use` or `extern crate` declarations.

**Fix:**
```bash
grep -n "extern crate alloc" src/lib.rs
# Remove the duplicate line
```

### Linker errors for bare metal target

```bash
rustup component add rust-src
cargo +nightly build --target x86_64-unknown-none -Z build-std=core,alloc
```

### `cargo check` takes too long

Use `cargo check --package sigma-kernel` to check only the specific package you're working on.

## QEMU Issues

### QEMU not found
```bash
sudo apt install qemu-system-x86   # Ubuntu/Debian
sudo pacman -S qemu-full            # Arch
sudo dnf install qemu-system-x86   # Fedora
```

### Boot fails in QEMU
- Ensure the ISO was built successfully: `ls -la build/SigmaOS.iso`
- Try with more RAM: `make run RAM=4G`
- Try without KVM: `make run KVM=0`

## Test Failures

### Python tests fail
```bash
pip3 install pytest
python3 -m pytest tests/ -v
```

### `run_sigma_tests.sh` fails at test 2

Type definitions from certain branches may be missing. The test script has been updated to not abort on individual test failures.

## Getting Help

- **GitHub Issues:** https://github.com/AaryanSinghChauhan09/SigmaOS/issues
- **Discussions:** https://github.com/AaryanSinghChauhan09/SigmaOS/discussions
- **Wiki:** https://github.com/AaryanSinghChauhan09/SigmaOS/wiki
