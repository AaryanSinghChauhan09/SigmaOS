# Docker Setup for SigmaOS Build

## Immediate Steps Required

Since you're on Linux Mint, Docker needs to be started and your user needs access.

### Option 1: Start Docker (requires sudo password)

```bash
# Start Docker daemon
sudo systemctl start docker

# Enable Docker to start on boot
sudo systemctl enable docker

# Add your user to docker group
sudo usermod -aG docker $USER

# CRITICAL: Log out and log back in for group changes to take effect
```

After logging back in, verify:

```bash
# Check if docker group is active
groups

# Test Docker access
docker ps
```

If `docker ps` works without sudo, proceed to build:

```bash
cd /home/aaryansinghchauhan/SigmaOS
scripts/build_archiso.sh --docker
```

### Option 2: Use Arch Linux VM

If Docker setup is problematic, use an Arch Linux VM:

```bash
# On Arch Linux VM
sudo pacman -S archiso
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
sudo scripts/build_archiso.sh --native
```

### Option 3: Use GitHub Actions (Automated)

I can create a GitHub Actions workflow to build the ISO automatically. This would:
- Build on Ubuntu runner with Docker
- Upload the ISO as an artifact
- No local Docker setup required

Would you like me to create the GitHub Actions workflow?

## Current Blocker

**Error**: Docker daemon is not running or current user lacks access.

**Solution**: Run the commands in Option 1 above, then log out and back in.

## After Successful Build

Once the ISO builds, test it in QEMU:

```bash
# Create test disk
qemu-img create -f qcow2 /tmp/sigmaos_test.qcow2 32G

# Boot ISO
qemu-system-x86_64 \
  -m 4096 \
  -enable-kvm \
  -drive file=/tmp/sigmaos_test.qcow2,format=qcow2,if=virtio \
  -cdrom out/sigmaos-desktop-YYYYMMDD-x86_64.iso \
  -boot d
```
