# SigmaOS Troubleshooting Guide

This document provides solutions to common issues encountered when building, running, or developing SigmaOS.

## Build Issues

### Cargo Build Fails

**Problem**: `cargo build` fails with compilation errors

**Solutions**:
1. Check Rust version:
   ```bash
   rustc --version
   ```
   Ensure you have Rust 1.70 or later

2. Update Rust toolchain:
   ```bash
   rustup update
   ```

3. Clean and rebuild:
   ```bash
   cargo clean
   cargo build --release
   ```

4. Check for missing targets:
   ```bash
   rustup target add x86_64-unknown-none
   ```

### Linker Errors

**Problem**: Linker errors during build

**Solutions**:
1. Install binutils:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install binutils-x86-64-linux-gnu

   # Fedora
   sudo dnf install binutils
   ```

2. Use system linker:
   ```bash
   export RUSTFLAGS="-C linker=clang"
   ```

### Missing Dependencies

**Problem**: Cargo complains about missing dependencies

**Solutions**:
1. Update cargo index:
   ```bash
   cargo update
   ```

2. Clean cargo cache:
   ```bash
   cargo clean
   rm -rf ~/.cargo/registry
   cargo build
   ```

## Boot Issues

### QEMU Won't Boot ISO

**Problem**: QEMU fails to boot the SigmaOS ISO

**Solutions**:
1. Verify ISO exists:
   ```bash
   ls -lh sigmaos.iso
   ```

2. Rebuild ISO:
   ```bash
   ./scripts/build-iso.sh
   ```

3. Check QEMU version:
   ```bash
   qemu-system-x86_64 --version
   ```
   Ensure QEMU 7.0 or later

4. Try with more memory:
   ```bash
   qemu-system-x86_64 -cdrom sigmaos.iso -m 4G
   ```

5. Enable serial output for debugging:
   ```bash
   qemu-system-x86_64 -cdrom sigmaos.iso -m 2G -serial stdio
   ```

### Real Hardware Won't Boot

**Problem**: SigmaOS won't boot on real hardware

**Solutions**:
1. Verify UEFI support:
   - Check BIOS settings for UEFI mode
   - Disable Legacy/CSM boot

2. Check Secure Boot:
   - Disable Secure Boot temporarily
   - Or sign the bootloader

3. Verify boot order:
   - Set USB/CD as first boot device

4. Try different USB ports:
   - USB 2.0 ports are more compatible
   - Avoid USB 3.0 ports if possible

### Kernel Panic

**Problem**: Kernel panics during boot

**Solutions**:
1. Enable kernel debugging:
   ```bash
   qemu-system-x86_64 -cdrom sigmaos.iso -m 2G -s -S
   gdb target/kernel
   (gdb) target remote :1234
   (gdb) break kernel_main
   (gdb) continue
   ```

2. Check memory map:
   - Verify bootloader passes correct memory map
   - Check for memory region conflicts

3. Verify kernel entry point:
   - Ensure kernel is loaded at correct address
   - Check ELF entry point matches expected

## Runtime Issues

### System Freezes

**Problem**: System freezes during operation

**Solutions**:
1. Check interrupt handling:
   - Verify APIC/PIC initialization
   - Check interrupt handlers are registered

2. Enable debug output:
   - Add serial console logging
   - Print debug messages at key points

3. Check scheduler:
   - Verify task switching works
   - Check for deadlocks

### Memory Allocation Fails

**Problem**: Memory allocation returns errors

**Solutions**:
1. Check buddy allocator:
   - Verify initialization
   - Check free list integrity

2. Increase available memory:
   ```bash
   qemu-system-x86_64 -cdrom sigmaos.iso -m 4G
   ```

3. Check for memory leaks:
   - Verify allocations are freed
   - Check for double-free bugs

### Filesystem Errors

**Problem**: File operations fail

**Solutions**:
1. Check VFS initialization:
   - Verify filesystem is mounted
   - Check root directory exists

2. Verify disk I/O:
   - Check disk driver is loaded
   - Verify disk is accessible

3. Check permissions:
   - Verify capability checks
   - Check file permissions

## Development Issues

### Tests Fail

**Problem**: Unit tests fail

**Solutions**:
1. Run tests individually:
   ```bash
   cargo test test_name
   ```

2. Enable test output:
   ```bash
   cargo test -- --nocapture
   ```

3. Enable backtrace:
   ```bash
   RUST_BACKTRACE=1 cargo test
   ```

4. Check test environment:
   - Verify test dependencies
   - Check test configuration

### Clippy Warnings

**Problem**: `cargo clippy` shows warnings

**Solutions**:
1. Fix clippy suggestions:
   ```bash
   cargo clippy --fix
   ```

2. Allow specific lints:
   ```rust
   #[allow(clippy::too_many_arguments)]
   fn function_with_many_args(...) { }
   ```

3. Update clippy:
   ```bash
   rustup update
   ```

### Documentation Build Fails

**Problem**: `cargo doc` fails

**Solutions**:
1. Check for broken links:
   ```bash
   cargo doc --document-private-items
   ```

2. Fix documentation comments:
   - Ensure all public items have docs
   - Fix broken markdown

3. Check external dependencies:
   - Some crates may have missing docs

## Network Issues

### Network Not Working

**Problem**: Network connectivity fails

**Solutions**:
1. Check network driver:
   - Verify driver is loaded
   - Check device is recognized

2. Configure network:
   ```bash
   # Set IP address
   ip addr add 192.168.1.100/24 dev eth0
   ip link set eth0 up
   ```

3. Check routing:
   ```bash
   ip route add default via 192.168.1.1
   ```

4. Test connectivity:
   ```bash
   ping 8.8.8.8
   ```

### WiFi Not Connecting

**Problem**: WiFi connection fails

**Solutions**:
1. Check WiFi driver:
   - Verify iwlwifi or mt7921 driver loaded
   - Check firmware is present

2. Scan for networks:
   ```bash
   iw dev wlan0 scan
   ```

3. Connect to network:
   ```bash
   wpa_supplicant -B -i wlan0 -c /etc/wpa_supplicant.conf
   ```

## Graphics Issues

### No Display Output

**Problem**: No graphics output

**Solutions**:
1. Check framebuffer:
   - Verify GOP/VESA initialization
   - Check framebuffer address

2. Try text mode:
   - Disable framebuffer in bootloader
   - Use VGA text mode

3. Check GPU driver:
   - Verify DRM/KMS driver loaded
   - Check GPU is recognized

### Display Corruption

**Problem**: Display shows corrupted graphics

**Solutions**:
1. Check framebuffer format:
   - Verify pixel format (RGB vs BGR)
   - Check bits per pixel

2. Verify stride:
   - Ensure stride matches width
   - Check for alignment issues

3. Test with different resolutions:
   - Try lower resolution
   - Check for memory bandwidth issues

## Security Issues

### Capability Check Fails

**Problem**: Capability checks deny legitimate operations

**Solutions**:
1. Check capability set:
   - Verify process has required capabilities
   - Check capability derivation

2. Review security policy:
   - Check MAC profile configuration
   - Verify Landlock rules

3. Debug capability system:
   - Enable capability audit logging
   - Check capability derivation tree

### Permission Denied

**Problem**: File operations return permission denied

**Solutions**:
1. Check file permissions:
   - Verify file mode bits
   - Check ownership

2. Verify capabilities:
   - Check for CAP_CHOWN, CAP_DAC_OVERRIDE
   - Verify capability set is correct

3. Review MAC policy:
   - Check Landlock filesystem rules
   - Verify seccomp filters

## Performance Issues

### System Slow

**Problem**: System performance is poor

**Solutions**:
1. Check scheduler:
   - Verify time slice quantum
   - Check for priority inversion

2. Profile system:
   ```bash
   cargo flamegraph
   ```

3. Optimize hot paths:
   - Use inline assembly for critical code
   - Reduce allocations in hot paths

### High Memory Usage

**Problem**: System uses too much memory

**Solutions**:
1. Check for memory leaks:
   - Verify allocations are freed
   - Check for reference cycles

2. Optimize data structures:
   - Use smaller types where possible
   - Reduce padding in structs

3. Enable memory compression:
   - Implement zswap/zram
   - Compress inactive pages

## Getting Help

If you encounter issues not covered here:

1. Check existing GitHub Issues:
   https://github.com/AaryanSinghChauhan09/SigmaOS/issues

2. Search the Wiki:
   https://github.com/AaryanSinghChauhan09/SigmaOS/wiki

3. Create a new Issue:
   - Include system information
   - Provide error messages
   - Describe steps to reproduce
   - Include debug logs if available

4. Join Discussions:
   https://github.com/AaryanSinghChauhan09/SigmaOS/discussions

## Debugging Tips

### Enable Debug Output

Add debug prints to code:
```rust
#[cfg(debug_assertions)]
println!("Debug: {}", value);
```

### Use Serial Console

Enable serial output in QEMU:
```bash
qemu-system-x86_64 -cdrom sigmaos.iso -serial stdio
```

### Use GDB

Debug with GDB:
```bash
qemu-system-x86_64 -cdrom sigmaos.iso -s -S
gdb target/kernel
(gdb) target remote :1234
```

### Check Logs

Review system logs:
```bash
# View kernel log
cat /var/log/kernel.log

# View system log
cat /var/log/syslog
```

## Common Error Messages

### "Out of Memory"

- Increase available memory
- Check for memory leaks
- Reduce memory usage

### "Permission Denied"

- Check file permissions
- Verify capabilities
- Review security policy

### "Device Not Found"

- Check driver is loaded
- Verify device is recognized
- Check device tree

### "Connection Refused"

- Check network configuration
- Verify firewall rules
- Check service is running
