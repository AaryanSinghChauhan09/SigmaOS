#!/bin/bash
# SigmaOS ISO Build Script
# Builds a bootable SigmaOS ISO image

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ISO_DIR="${SCRIPT_DIR}/iso"
OUTPUT_DIR="${SCRIPT_DIR}/output"
ISO_FILE="${OUTPUT_DIR}/sigmaos.iso"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check dependencies
check_dependencies() {
    log_info "Checking dependencies..."
    
    local deps=("xorriso" "grub-mkrescue" "mksquashfs")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            log_warn "$dep not found, attempting to install..."
            sudo apt-get update && sudo apt-get install -y xorriso grub-pc-bin squashfs-tools || true
        fi
    done
}

# Create directory structure
create_structure() {
    log_info "Creating directory structure..."
    mkdir -p "${OUTPUT_DIR}"
    mkdir -p "${ISO_DIR}/boot/grub"
    mkdir -p "${ISO_DIR}/EFI/BOOT"
    mkdir -p "${ISO_DIR}/live"
}

# Copy GRUB configuration
copy_grub_config() {
    log_info "Copying GRUB configuration..."
    cp "${SCRIPT_DIR}/grub.cfg" "${ISO_DIR}/boot/grub/grub.cfg"
}

# Create minimal kernel stub (for demonstration)
create_kernel_stub() {
    log_info "Creating kernel stub..."
    
    # Check if compiled kernel stub exists
    if [ -f "${SCRIPT_DIR}/kernel-stub" ]; then
        log_info "Using compiled kernel stub..."
        cp "${SCRIPT_DIR}/kernel-stub" "${ISO_DIR}/boot/vmlinuz"
    else
        # Fallback to shell script stub
        log_warn "Compiled kernel not found, using script stub..."
        echo "#!/bin/bash" > "${ISO_DIR}/boot/vmlinuz"
        echo "echo 'SigmaOS Kernel Stub v0.1.0'" >> "${ISO_DIR}/boot/vmlinuz"
        echo "echo 'Kernel initialization started...'" >> "${ISO_DIR}/boot/vmlinuz"
        echo "echo 'This is a stub - actual kernel implementation in progress'" >> "${ISO_DIR}/boot/vmlinuz"
        chmod +x "${ISO_DIR}/boot/vmlinuz"
    fi
}

# Create initramfs stub
create_initramfs() {
    log_info "Creating initramfs stub..."
    # Create a minimal initramfs structure
    local initramfs_dir="${SCRIPT_DIR}/initramfs-temp"
    mkdir -p "${initramfs_dir}/bin"
    mkdir -p "${initramfs_dir}/dev"
    mkdir -p "${initramfs_dir}/proc"
    mkdir -p "${initramfs_dir}/sys"
    mkdir -p "${initramfs_dir}/mnt"
    
    # Create init script
    cat > "${initramfs_dir}/init" << 'EOF'
#!/bin/sh
# SigmaOS init script

echo "SigmaOS Initializing..."
mount -t proc none /proc
mount -t sysfs none /sys
mount -t devtmpfs none /dev

echo "SigmaOS Initramfs Loaded"
exec /bin/sh
EOF
    chmod +x "${initramfs_dir}/init"
    
    # Create initramfs image
    cd "${initramfs_dir}"
    find . | cpio -o -H newc | gzip > "${ISO_DIR}/boot/initramfs.img"
    cd "${SCRIPT_DIR}"
    
    # Cleanup
    rm -rf "${initramfs_dir}"
}

# Create squashfs root filesystem
create_rootfs() {
    log_info "Creating root filesystem..."
    local rootfs_dir="${SCRIPT_DIR}/rootfs-temp"
    
    # Clean up any existing rootfs
    rm -rf "${rootfs_dir}"
    rm -f "${ISO_DIR}/live/filesystem.squashfs"
    
    mkdir -p "${rootfs_dir}"
    
    # Create basic directory structure
    mkdir -p "${rootfs_dir}/bin"
    mkdir -p "${rootfs_dir}/sbin"
    mkdir -p "${rootfs_dir}/usr/bin"
    mkdir -p "${rootfs_dir}/usr/sbin"
    mkdir -p "${rootfs_dir}/usr/lib"
    mkdir -p "${rootfs_dir}/etc"
    mkdir -p "${rootfs_dir}/var"
    mkdir -p "${rootfs_dir}/var/log"
    mkdir -p "${rootfs_dir}/var/cache"
    mkdir -p "${rootfs_dir}/tmp"
    mkdir -p "${rootfs_dir}/home"
    mkdir -p "${rootfs_dir}/root"
    mkdir -p "${rootfs_dir}/dev"
    mkdir -p "${rootfs_dir}/proc"
    mkdir -p "${rootfs_dir}/sys"
    mkdir -p "${rootfs_dir}/mnt"
    mkdir -p "${rootfs_dir}/opt"
    mkdir -p "${rootfs_dir}/srv"
    
    # Create SigmaOS-specific files
    cat > "${rootfs_dir}/etc/os-release" << 'EOF'
NAME="SigmaOS"
ID=sigmaos
VERSION="0.1.0"
VERSION_ID="0.1"
PRETTY_NAME="SigmaOS 0.1.0"
ID_LIKE=arch
ANSI_COLOR="38;2;23;147;209"
HOME_URL="https://github.com/AaryanSinghChauhan09/SigmaOS"
DOCUMENTATION_URL="https://github.com/AaryanSinghChauhan09/SigmaOS/wiki"
SUPPORT_URL="https://github.com/AaryanSinghChauhan09/SigmaOS/issues"
BUG_REPORT_URL="https://github.com/AaryanSinghChauhan09/SigmaOS/issues"
PRIVACY_POLICY_URL="https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Privacy"
EOF
    
    cat > "${rootfs_dir}/etc/hostname" << 'EOF'
sigmaos
EOF
    
    # Create package manager configuration
    mkdir -p "${rootfs_dir}/etc/sigmaos"
    cat > "${rootfs_dir}/etc/sigmaos/pacman.conf" << 'EOF'
[options]
HoldPkg     = pacman glibc
Architecture = auto
CheckSpace
SigLevel    = Required
LocalFileSigLevel = Optional

[core]
Include = /etc/sigmaos/pacman.d/mirrorlist

[extra]
Include = /etc/sigmaos/pacman.d/mirrorlist

[community]
Include = /etc/sigmaos/pacman.d/mirrorlist
EOF
    
    mkdir -p "${rootfs_dir}/etc/sigmaos/pacman.d"
    cat > "${rootfs_dir}/etc/sigmaos/pacman.d/mirrorlist" << 'EOF'
## SigmaOS Repository Mirrors
Server = https://sigmaos.org/packages/$repo/os/$arch
Server = https://mirror.sigmaos.org/$repo/os/$arch
EOF
    
    # Create basic package database
    mkdir -p "${rootfs_dir}/var/lib/pacman/local"
    cat > "${rootfs_dir}/var/lib/pacman/local/sigmaos-db" << 'EOF'
%NAME%
sigmaos-db
%VERSION%
0.1.0
%DESC%
SigmaOS package database
%URL%
https://github.com/AaryanSinghChauhan09/SigmaOS
%LICENSE%
MIT
EOF
    
    # Create basic shell scripts
    cat > "${rootfs_dir}/bin/sigmaos-shell" << 'EOF'
#!/bin/sh
# SigmaOS Shell
echo "SigmaOS Shell v0.1.0"
echo "Type 'help' for available commands"
while true; do
    printf "sigmaos> "
    read cmd
    case "$cmd" in
        help)
            echo "Available commands:"
            echo "  help    - Show this help"
            echo "  version - Show SigmaOS version"
            echo "  exit    - Exit shell"
            ;;
        version)
            cat /etc/os-release
            ;;
        exit)
            echo "Goodbye!"
            exit 0
            ;;
        "")
            ;;
        *)
            echo "Unknown command: $cmd"
            ;;
    esac
done
EOF
    chmod +x "${rootfs_dir}/bin/sigmaos-shell"
    
    # Create squashfs
    mksquashfs "${rootfs_dir}" "${ISO_DIR}/live/filesystem.squashfs" -comp xz -noappend
    
    # Cleanup
    rm -rf "${rootfs_dir}"
}

# Build ISO
build_iso() {
    log_info "Building ISO image..."
    
    # Try genisoimage first (available on this system)
    if command -v genisoimage &> /dev/null; then
        log_info "Using genisoimage..."
        genisoimage \
            -r -J -joliet-long \
            -b boot/grub/grub.cfg \
            -c boot.cat \
            -no-emul-boot \
            -boot-load-size 4 \
            -boot-info-table \
            -o "${ISO_FILE}" \
            "${ISO_DIR}"
    elif command -v xorriso &> /dev/null; then
        log_info "Using xorriso..."
        xorriso -as mkisofs \
            -r -J -joliet-long \
            -b boot/grub/grub.cfg \
            -c boot.cat \
            -no-emul-boot \
            -boot-load-size 4 \
            -boot-info-table \
            -o "${ISO_FILE}" \
            "${ISO_DIR}"
    else
        log_error "Neither genisoimage nor xorriso found. Cannot build ISO."
        return 1
    fi
    
    log_info "ISO built successfully: ${ISO_FILE}"
}

# Main build process
main() {
    log_info "Starting SigmaOS ISO build..."
    
    check_dependencies
    create_structure
    copy_grub_config
    create_kernel_stub
    create_initramfs
    create_rootfs
    build_iso
    
    log_info "Build complete!"
    log_info "ISO location: ${ISO_FILE}"
    log_info "To test in QEMU: qemu-system-x86_64 -cdrom ${ISO_FILE} -m 2048"
}

main "$@"
