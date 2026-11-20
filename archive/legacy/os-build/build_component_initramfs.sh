#!/usr/bin/env bash
# SigmaOS Component Initramfs Stager
#
# Scans sibling SigmaOS-* component repositories and stages their build
# artifacts + COMPONENT.toml metadata into an initramfs layout:
#
#   <stage>/sigmaos/components/<name>/COMPONENT.toml
#   <stage>/sigmaos/components/<name>/lib<name>.rlib   (if prebuilt)
#
# This is the mechanism half of the multi-repo -> initramfs pipeline:
# mkinitramfs.sh packages <stage> into the final CPIO image.
#
# Honest scope: this stages *metadata and prebuilt artifacts*. It does
# NOT cross-compile components for x86_64-unknown-none — that requires
# each component to be a #![no_std] crate and is tracked separately.
set -euo pipefail

STAGE_DIR="${1:-/tmp/sigmaos-initramfs-stage}"
COMPONENTS_ROOT="${SIGMAOS_COMPONENTS_ROOT:-$HOME}"
DRY_RUN=0
[ "${2:-}" = "--dry-run" ] && DRY_RUN=1

mkdir -p "${STAGE_DIR}/sigmaos/components"
MANIFEST="${STAGE_DIR}/sigmaos/components/MANIFEST"
: > "${MANIFEST}"

count=0
skipped=0
for dir in "${COMPONENTS_ROOT}"/SigmaOS-*/; do
    name="$(basename "${dir}")"
    comp_toml="${dir}COMPONENT.toml"
    [ -f "${comp_toml}" ] || { skipped=$((skipped+1)); continue; }

    dest="${STAGE_DIR}/sigmaos/components/${name}"
    if [ "${DRY_RUN}" = "0" ]; then
        mkdir -p "${dest}"
        cp "${comp_toml}" "${dest}/COMPONENT.toml"
        # Stage prebuilt rlib if the component was cargo-built for the
        # bare-metal target; skip silently otherwise (modeled vs built
        # distinction is recorded in the manifest).
        rlib="$(find "${dir}target" -name 'lib*.rlib' 2>/dev/null | head -1 || true)"
        if [ -n "${rlib}" ]; then
            cp "${rlib}" "${dest}/" 2>/dev/null || true
            echo "${name} artifact" >> "${MANIFEST}"
        else
            echo "${name} metadata-only" >> "${MANIFEST}"
        fi
    fi
    count=$((count+1))
done

echo "Staged ${count} components into ${STAGE_DIR} (${skipped} skipped: no COMPONENT.toml)"
echo "Artifact-bearing: $(grep -c 'artifact$' "${MANIFEST}" 2>/dev/null || echo 0)"
echo "Next: scripts/mkinitramfs.sh -d ${STAGE_DIR} -o /boot/initramfs-sigma.img"
