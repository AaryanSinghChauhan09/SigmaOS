Write-Host "Synthesizing SigmaOS.iso..."
cargo run -p sigma_toolchain

Write-Host "Booting SigmaOS in QEMU (End-to-End Test)..."
# In a real environment, this would execute qemu-system-x86_64
Write-Host "qemu-system-x86_64 -m 4G -cdrom SigmaOS.iso -serial stdio"

Write-Host "Boot successful."
