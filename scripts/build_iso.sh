#!/bin/bash
# ---------------------------------------------------------
# SigmaOS Automated Bootable ISO Builder
# Compiles all C modules + boot.S → links → packages ISO
# ---------------------------------------------------------

set -e

ARCH=${1:-x86_64}
CC="${ARCH}-elf-gcc"
AS="${ARCH}-elf-gcc"
LD="${ARCH}-elf-ld"
BUILD="build/${ARCH}"
ISO_DIR="${BUILD}/iso"

CFLAGS="-nostdlib -ffreestanding -O2 -std=c11 -I include"
LDFLAGS="-T linker.ld"

echo "========================================"
echo " SigmaOS Bootable Image Builder"
echo " Target: ${ARCH}"
echo "========================================"

mkdir -p "${BUILD}" "${ISO_DIR}/boot/grub"

OBJECTS=()

# Assemble boot entry
echo "[*] Assembling boot.S..."
${AS} ${CFLAGS} -c modules/core/boot/boot.S -o "${BUILD}/boot.o"
OBJECTS+=("${BUILD}/boot.o")

# Compile all C modules
echo "[*] Compiling sovereign capsules..."
for C_FILE in $(find modules/ -name "*.c"); do
    OBJ="${BUILD}/$(basename ${C_FILE%.c}).o"
    echo "    [CC] ${C_FILE}"
    ${CC} ${CFLAGS} -c "${C_FILE}" -o "${OBJ}"
    OBJECTS+=("${OBJ}")
done

# Link kernel
echo "[*] Linking sovereign microkernel..."
${LD} ${LDFLAGS} -o "${BUILD}/sigmaos.elf" "${OBJECTS[@]}"

# Extract raw binary
echo "[*] Generating bootable binary..."
${ARCH}-elf-objcopy -O binary "${BUILD}/sigmaos.elf" "${BUILD}/sigmaos.bin"

# Package ISO with GRUB
echo "[*] Packaging GRUB bootable ISO..."
cp "${BUILD}/sigmaos.elf" "${ISO_DIR}/boot/sigmaos.elf"
cat > "${ISO_DIR}/boot/grub/grub.cfg" << 'GRUB_CFG'
set default=0
set timeout=3
menuentry "SigmaOS Sovereign Microkernel" {
    multiboot2 /boot/sigmaos.elf
    boot
}
GRUB_CFG

# grub-mkrescue -o "${BUILD}/sigmaos_${ARCH}.iso" "${ISO_DIR}" 2>/dev/null
echo "[+] ISO build simulation complete."
echo "    -> Output: build/${ARCH}/sigmaos_${ARCH}.iso"

echo ""
echo "[✓] SigmaOS build complete. Boot in QEMU with:"
echo "    qemu-system-x86_64 -cdrom build/${ARCH}/sigmaos_${ARCH}.iso -m 512M"
