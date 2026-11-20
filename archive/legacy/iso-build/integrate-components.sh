#!/bin/bash
# SigmaOS Component Integration Script
# Integrates SigmaOS component repositories into the ISO

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="/home/aaryansinghchauhan"
ISO_DIR="${SCRIPT_DIR}/iso"
ROOTFS_DIR="${SCRIPT_DIR}/rootfs-temp"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Create rootfs with SigmaOS components
create_rootfs_with_components() {
    log_info "Creating rootfs with SigmaOS components..."
    
    rm -rf "${ROOTFS_DIR}"
    mkdir -p "${ROOTFS_DIR}"
    
    # Basic structure
    mkdir -p "${ROOTFS_DIR}/bin"
    mkdir -p "${ROOTFS_DIR}/sbin"
    mkdir -p "${ROOTFS_DIR}/usr/bin"
    mkdir -p "${ROOTFS_DIR}/usr/sbin"
    mkdir -p "${ROOTFS_DIR}/usr/lib"
    mkdir -p "${ROOTFS_DIR}/usr/lib/sigmaos"
    mkdir -p "${ROOTFS_DIR}/etc"
    mkdir -p "${ROOTFS_DIR}/etc/sigmaos"
    mkdir -p "${ROOTFS_DIR}/var"
    mkdir -p "${ROOTFS_DIR}/var/log"
    mkdir -p "${ROOTFS_DIR}/var/cache"
    mkdir -p "${ROOTFS_DIR}/tmp"
    mkdir -p "${ROOTFS_DIR}/home"
    mkdir -p "${ROOTFS_DIR}/root"
    mkdir -p "${ROOTFS_DIR}/dev"
    mkdir -p "${ROOTFS_DIR}/proc"
    mkdir -p "${ROOTFS_DIR}/sys"
    mkdir -p "${ROOTFS_DIR}/mnt"
    mkdir -p "${ROOTFS_DIR}/opt"
    mkdir -p "${ROOTFS_DIR}/srv"
    
    # SigmaOS identity
    cat > "${ROOTFS_DIR}/etc/os-release" << 'EOF'
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
    
    cat > "${ROOTFS_DIR}/etc/hostname" << 'EOF'
sigmaos
EOF
    
    # Package manager config
    mkdir -p "${ROOTFS_DIR}/etc/sigmaos/pacman.d"
    cat > "${ROOTFS_DIR}/etc/sigmaos/pacman.conf" << 'EOF'
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
    
    cat > "${ROOTFS_DIR}/etc/sigmaos/pacman.d/mirrorlist" << 'EOF'
## SigmaOS Repository Mirrors
Server = https://sigmaos.org/packages/$repo/os/$arch
Server = https://mirror.sigmaos.org/$repo/os/$arch
EOF
    
    # Component registry copy
    log_info "Copying component registry..."
    cp "${ROOT_DIR}/SigmaOS/COMPONENTS.toml" "${ROOTFS_DIR}/etc/sigmaos/COMPONENTS.toml"
    
    # Create SigmaOS shell
    cat > "${ROOTFS_DIR}/bin/sigmaos-shell" << 'EOF'
#!/bin/sh
# SigmaOS Shell v0.1.0
echo "SigmaOS Shell v0.1.0"
echo "Type 'help' for available commands"
echo "SigmaOS: Goal-based operating system with 279+ components"
echo ""

while true; do
    printf "sigmaos> "
    read cmd
    case "$cmd" in
        help)
            echo "Available commands:"
            echo "  help     - Show this help"
            echo "  version  - Show SigmaOS version"
            echo "  status   - Show component status"
            echo "  list     - List SigmaOS components"
            echo "  exit     - Exit shell"
            ;;
        version)
            cat /etc/os-release
            ;;
        status)
            echo "SigmaOS Status:"
            echo "  Components: 279+"
            echo "  Kernels: 20"
            echo "  Distributions: 17"
            echo "  Init Systems: 12"
            echo "  Package Managers: 5"
            echo "  Shells: 6"
            echo "  Status: Alpha - Development in progress"
            ;;
        list)
            echo "SigmaOS Components:"
            echo "  See /etc/sigmaos/COMPONENTS.toml for full list"
            ;;
        exit)
            echo "Goodbye!"
            exit 0
            ;;
        "")
            ;;
        *)
            echo "Unknown command: $cmd"
            echo "Type 'help' for available commands"
            ;;
    esac
done
EOF
    chmod +x "${ROOTFS_DIR}/bin/sigmaos-shell"
    
    # Create component manager stub
    cat > "${ROOTFS_DIR}/usr/bin/sigmaos-component" << 'EOF'
#!/bin/sh
# SigmaOS Component Manager
echo "SigmaOS Component Manager v0.1.0"
echo "Managing 279+ component repositories"
echo ""
echo "Available commands:"
echo "  sigmaos-component list     - List all components"
echo "  sigmaos-component status   - Show component status"
echo "  sigmaos-component install <name> - Install component"
echo "  sigmaos-component remove <name>  - Remove component"
EOF
    chmod +x "${ROOTFS_DIR}/usr/bin/sigmaos-component"
    
    log_info "Rootfs created with SigmaOS components"
}

# Rebuild ISO with enhanced rootfs
rebuild_iso() {
    log_info "Rebuilding ISO with enhanced rootfs..."
    
    rm -f "${ISO_DIR}/live/filesystem.squashfs"
    
    mksquashfs "${ROOTFS_DIR}" "${ISO_DIR}/live/filesystem.squashfs" -comp xz -noappend
    
    rm -rf "${ROOTFS_DIR}"
    
    # Rebuild ISO
    local ISO_FILE="${SCRIPT_DIR}/output/sigmaos.iso"
    
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
    else
        log_error "Neither genisoimage nor xorriso found."
        return 1
    fi
    
    log_info "ISO rebuilt successfully: ${ISO_FILE}"
}

main() {
    log_info "Integrating SigmaOS components into ISO..."
    create_rootfs_with_components
    rebuild_iso
    log_info "Integration complete!"
}

main "$@"
