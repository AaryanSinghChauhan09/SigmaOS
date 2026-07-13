#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# scripts/catalogue_builder.py — Driver Catalogue Auto-Generator
#
# Parses the upstream Linux kernel tree to auto-generate driver_catalogue.json.
# Extracts: MODULE_DEVICE_TABLE entries, Kconfig descriptions, driver metadata.
#
# Usage:
#   python catalogue_builder.py --linux-tree /path/to/linux --output data/driver_catalogue.json
#   python catalogue_builder.py --test-mode   # generate from built-in seed data
#
# Requires: Python 3.8+, git (for deprecated driver recovery)

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple

# ── Well-known driver subsystems and their categories ──────────────────────

SUBSYSTEM_CATEGORY_MAP = {
    "drivers/net/ethernet":     "network",
    "drivers/net/wireless":     "wireless",
    "drivers/net/usb":          "network",
    "drivers/net/phy":          "network",
    "drivers/bluetooth":        "bluetooth",
    "drivers/nvme":             "storage",
    "drivers/ata":              "storage",
    "drivers/scsi":             "storage",
    "drivers/mmc":              "storage",
    "drivers/block":            "storage",
    "drivers/gpu/drm":          "gpu",
    "drivers/video":            "display",
    "drivers/sound":            "audio",
    "drivers/input":            "input",
    "drivers/hid":              "input",
    "drivers/usb":              "usb",
    "drivers/tty/serial":       "serial",
    "drivers/i2c":              "i2c",
    "drivers/spi":              "spi",
    "drivers/iio":              "sensor",
    "drivers/media":            "camera",
    "drivers/crypto":           "crypto",
    "drivers/watchdog":         "watchdog",
    "drivers/power":            "power",
    "drivers/acpi":             "power",
    "drivers/platform":         "platform",
    "drivers/virtio":           "virtio",
    "drivers/firmware":         "firmware",
    "drivers/infiniband":       "infiniband",
    "drivers/staging":          "misc",
}

# ── PCI vendor names (top vendors) ────────────────────────────────────────

PCI_VENDOR_NAMES = {
    0x8086: "Intel",
    0x10DE: "NVIDIA",
    0x1002: "AMD",
    0x14E4: "Broadcom",
    0x10EC: "Realtek",
    0x168C: "Qualcomm Atheros",
    0x1969: "Qualcomm Atheros",
    0x17CB: "Qualcomm",
    0x8087: "Intel (Wireless)",
    0x1B4B: "Marvell",
    0x11AB: "Marvell",
    0x15B3: "Mellanox",
    0x1AF4: "Red Hat (Virtio)",
    0x1D6B: "Linux Foundation",
    0x1A2B: "Ralink",
    0x148F: "Ralink/MediaTek",
    0x14C3: "MediaTek",
    0x1000: "LSI/Broadcom",
    0x19E5: "Huawei",
    0x177D: "Cavium",
    0x1077: "QLogic",
    0x15AD: "VMware",
    0x1AB4: "Samsung",
    0x144D: "Samsung",
}


def detect_category(filepath: str) -> str:
    """Determine driver category from its path in the kernel tree."""
    filepath_posix = filepath.replace("\\", "/")
    for prefix, category in sorted(SUBSYSTEM_CATEGORY_MAP.items(), key=lambda x: -len(x[0])):
        if filepath_posix.startswith(prefix):
            return category
    return "misc"


def extract_pci_ids(content: str) -> List[Dict]:
    """Extract PCI device IDs from MODULE_DEVICE_TABLE(pci, ...) entries."""
    hardware_ids = []

    # Pattern: PCI_DEVICE(vendor, device) or PCI_VDEVICE(vendor, device)
    pci_device_re = re.compile(
        r'PCI_(?:V?DEVICE|DEVICE_SUB)\s*\(\s*'
        r'(?:PCI_VENDOR_ID_)?(\w+)\s*,\s*'
        r'(?:0x)?([0-9A-Fa-f]{4})',
        re.MULTILINE
    )

    # Inline hex vendor/device: { 0x8086, 0x1234, ... }
    inline_re = re.compile(
        r'\{\s*(?:0x)?([0-9A-Fa-f]{4})\s*,\s*(?:0x)?([0-9A-Fa-f]{4})',
        re.MULTILINE
    )

    # Well-known vendor ID macros
    vendor_macro_map = {
        "INTEL": "8086",
        "REALTEK": "10EC",
        "NVIDIA": "10DE",
        "AMD": "1002",
        "ATI": "1002",
        "BROADCOM": "14E4",
        "ATHEROS": "168C",
        "QUALCOMM": "17CB",
        "MARVELL": "11AB",
        "MELLANOX": "15B3",
        "VIRTIO": "1AF4",
        "MEDIATEK": "14C3",
        "RALINK": "1A2B",
        "SAMSUNG": "144D",
    }

    for match in pci_device_re.finditer(content):
        vendor_name = match.group(1).upper()
        device_hex = match.group(2)
        vendor_hex = vendor_macro_map.get(vendor_name, None)
        if vendor_hex:
            hardware_ids.append({
                "bus": "pci",
                "vendor": f"0x{vendor_hex}",
                "device": f"0x{device_hex}",
            })

    for match in inline_re.finditer(content):
        vendor_hex = match.group(1)
        device_hex = match.group(2)
        vid = int(vendor_hex, 16)
        # Skip entries that are clearly not PCI IDs
        if vid != 0 and vid != 0xFFFF:
            hardware_ids.append({
                "bus": "pci",
                "vendor": f"0x{vendor_hex}",
                "device": f"0x{device_hex}",
            })

    return hardware_ids


def extract_usb_ids(content: str) -> List[Dict]:
    """Extract USB device IDs from MODULE_DEVICE_TABLE(usb, ...)."""
    hardware_ids = []

    usb_device_re = re.compile(
        r'USB_DEVICE\s*\(\s*(?:0x)?([0-9A-Fa-f]{4})\s*,\s*(?:0x)?([0-9A-Fa-f]{4})',
        re.MULTILINE
    )

    for match in usb_device_re.finditer(content):
        hardware_ids.append({
            "bus": "usb",
            "vendor": f"0x{match.group(1)}",
            "product": f"0x{match.group(2)}",
        })

    return hardware_ids


def extract_driver_name(content: str, filepath: str) -> str:
    """Extract driver name from source code or filename."""
    # Try DRV_NAME / DRIVER_NAME
    name_re = re.compile(r'#define\s+(?:DRV|DRIVER)_NAME\s+"([^"]+)"')
    match = name_re.search(content)
    if match:
        return match.group(1)

    # Try KBUILD_MODNAME
    mod_re = re.compile(r'MODULE_DESCRIPTION\s*\(\s*"([^"]+)"')
    match = mod_re.search(content)
    if match:
        return match.group(1)[:60]

    # Fallback to filename
    return Path(filepath).stem


def extract_kconfig_description(kconfig_path: str, module_name: str) -> str:
    """Parse Kconfig file to find description for a specific driver."""
    if not os.path.exists(kconfig_path):
        return ""

    try:
        with open(kconfig_path, "r", errors="replace") as f:
            content = f.read()
    except IOError:
        return ""

    # Find config block and extract help text
    config_re = re.compile(
        rf'config\s+{re.escape(module_name)}\s*\n(.*?)(?=\nconfig\s|\nendmenu|\Z)',
        re.DOTALL | re.IGNORECASE
    )
    match = config_re.search(content)
    if not match:
        return ""

    block = match.group(1)
    help_re = re.compile(r'(?:help|---help---)\s*\n((?:\s{2,}.*\n?)+)', re.MULTILINE)
    help_match = help_re.search(block)
    if help_match:
        lines = help_match.group(1).strip().split("\n")
        return " ".join(line.strip() for line in lines[:3])  # First 3 lines

    return ""


def extract_firmware_deps(content: str) -> List[Dict]:
    """Extract firmware dependencies from MODULE_FIRMWARE() macros."""
    deps = []
    fw_re = re.compile(r'MODULE_FIRMWARE\s*\(\s*"([^"]+)"', re.MULTILINE)
    for match in fw_re.finditer(content):
        deps.append({
            "name": match.group(1),
            "kind": "firmware",
            "source": f"https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git/tree/{match.group(1)}",
            "required": True,
        })
    return deps


def extract_license(content: str) -> str:
    """Extract SPDX license from source."""
    spdx_re = re.compile(r'SPDX-License-Identifier:\s*(.+?)(?:\s*\*/|\s*$)', re.MULTILINE)
    match = spdx_re.search(content)
    if match:
        return match.group(1).strip()

    lic_re = re.compile(r'MODULE_LICENSE\s*\(\s*"([^"]+)"')
    match = lic_re.search(content)
    if match:
        lic = match.group(1)
        return {"GPL": "GPL-2.0-only", "GPL v2": "GPL-2.0-only",
                "Dual BSD/GPL": "GPL-2.0-only OR BSD-2-Clause",
                "Dual MIT/GPL": "GPL-2.0-only OR MIT",
                "Proprietary": "Proprietary"}.get(lic, lic)

    return "GPL-2.0-only"


def scan_driver_file(filepath: str, linux_tree: str) -> Optional[Dict]:
    """Scan a single C source file and extract driver metadata."""
    try:
        with open(filepath, "r", errors="replace") as f:
            content = f.read()
    except IOError:
        return None

    # Must have MODULE_DEVICE_TABLE or pci_driver/usb_driver to be interesting
    if "MODULE_DEVICE_TABLE" not in content and "pci_driver" not in content:
        if "usb_driver" not in content and "platform_driver" not in content:
            return None

    rel_path = os.path.relpath(filepath, linux_tree).replace("\\", "/")
    category = detect_category(rel_path)
    driver_name = extract_driver_name(content, filepath)

    # Extract hardware IDs
    hw_ids = extract_pci_ids(content) + extract_usb_ids(content)
    if not hw_ids:
        return None  # Can't catalogue without hardware IDs

    # Build vendor name from first PCI ID
    vendor = "Unknown"
    for hw in hw_ids:
        if hw.get("bus") == "pci":
            vid = int(hw["vendor"], 16)
            vendor = PCI_VENDOR_NAMES.get(vid, f"Vendor 0x{vid:04X}")
            break
        elif hw.get("bus") == "usb":
            vendor = "USB Device"
            break

    # Extract firmware deps
    deps = extract_firmware_deps(content)

    # Kconfig description
    kconfig_path = os.path.join(os.path.dirname(filepath), "Kconfig")
    description = extract_kconfig_description(kconfig_path, driver_name.upper())
    if not description:
        description = f"{driver_name} driver for {vendor} hardware"

    # Generate stable ID
    driver_id = f"linux-{driver_name.lower().replace(' ', '-').replace('_', '-')}"

    # Determine chipset family from path
    path_parts = rel_path.split("/")
    chipset = path_parts[-2] if len(path_parts) > 2 else ""

    return {
        "id": driver_id,
        "display_name": driver_name,
        "description": description,
        "category": category,
        "status": "active",
        "compat_status": "untested",
        "hardware_ids": hw_ids[:20],  # Cap at 20 most specific IDs
        "min_kernel": "2.6.0",
        "kernel_path": os.path.dirname(rel_path),
        "vendor": vendor,
        "chipset_family": chipset,
        "dependencies": deps,
        "tags": [category, vendor.lower(), chipset.lower()],
        "license": extract_license(content),
        "maintainer": "",
    }


def scan_linux_tree(linux_tree: str) -> List[Dict]:
    """Walk the Linux kernel drivers/ tree and extract all driver metadata."""
    drivers = []
    drivers_dir = os.path.join(linux_tree, "drivers")

    if not os.path.isdir(drivers_dir):
        print(f"Error: {drivers_dir} is not a directory", file=sys.stderr)
        return drivers

    print(f"Scanning {drivers_dir}...")
    file_count = 0

    for root, dirs, files in os.walk(drivers_dir):
        # Skip test/debug directories
        dirs[:] = [d for d in dirs if d not in {"test", "tests", "selftests", ".git"}]

        for fname in files:
            if not fname.endswith(".c"):
                continue
            filepath = os.path.join(root, fname)
            file_count += 1

            if file_count % 500 == 0:
                print(f"  Scanned {file_count} files, found {len(drivers)} drivers...")

            entry = scan_driver_file(filepath, linux_tree)
            if entry:
                drivers.append(entry)

    print(f"Scan complete: {file_count} files → {len(drivers)} drivers catalogued")
    return drivers


def generate_seed_catalogue() -> Dict:
    """Generate a seed catalogue with well-known drivers for bootstrapping."""
    return {
        "version": "1.0.0",
        "generated": "2026-07-09T00:00:00Z",
        "source": "seed",
        "drivers": [
            # ── Network ──────────────────────────────────────────────
            {
                "id": "linux-e1000",
                "display_name": "Intel PRO/1000 (e1000)",
                "description": "Intel PRO/1000 Gigabit Ethernet driver. One of the most widely-used NIC drivers, supporting Intel 82540/82541/82542/82543/82544/82545/82546/82547 controllers.",
                "category": "network",
                "status": "active",
                "compat_status": "native",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x8086", "device": "0x100E"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x100F"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x1010"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x1011"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x1012"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x1019"},
                ],
                "min_kernel": "2.4.0",
                "kernel_path": "drivers/net/ethernet/intel/e1000",
                "vendor": "Intel",
                "chipset_family": "Intel 82540EM",
                "sigma_module": "sigma-e1000",
                "dependencies": [],
                "tags": ["network", "ethernet", "intel", "gigabit", "pci"],
                "license": "GPL-2.0-only",
                "maintainer": "Intel Wired Networking <e1000-devel@lists.sourceforge.net>",
            },
            {
                "id": "linux-e1000e",
                "display_name": "Intel PRO/1000 PCI-Express (e1000e)",
                "description": "Intel PRO/1000 PCIe Ethernet driver for 82563/82566/82567/82571/82572/82573/82574/82583 and ICH8-ICH10/PCH controllers.",
                "category": "network",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x8086", "device": "0x105E"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x10A4"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x10BD"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x10C0"},
                ],
                "min_kernel": "2.6.24",
                "kernel_path": "drivers/net/ethernet/intel/e1000e",
                "vendor": "Intel",
                "chipset_family": "Intel 82571/82572/ICH",
                "dependencies": [],
                "tags": ["network", "ethernet", "intel", "pcie"],
                "license": "GPL-2.0-only",
                "maintainer": "Intel Wired Networking",
            },
            {
                "id": "linux-r8169",
                "display_name": "Realtek RTL8169/RTL8168/RTL8101/RTL8125",
                "description": "Realtek Gigabit/2.5G Ethernet driver. Supports RTL8169, RTL8168, RTL8101E, RTL8125 families. Found on most consumer motherboards.",
                "category": "network",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x10EC", "device": "0x8168"},
                    {"bus": "pci", "vendor": "0x10EC", "device": "0x8169"},
                    {"bus": "pci", "vendor": "0x10EC", "device": "0x8136"},
                    {"bus": "pci", "vendor": "0x10EC", "device": "0x8125"},
                ],
                "min_kernel": "2.4.0",
                "kernel_path": "drivers/net/ethernet/realtek",
                "vendor": "Realtek",
                "chipset_family": "RTL8111/RTL8168",
                "dependencies": [
                    {"name": "rtl_nic/rtl8168h-2.fw", "kind": "firmware",
                     "source": "linux-firmware", "required": False}
                ],
                "tags": ["network", "ethernet", "realtek", "gigabit", "2.5gbe"],
                "license": "GPL-2.0-only",
                "maintainer": "Heiner Kallweit <hkallweit1@gmail.com>",
            },
            {
                "id": "linux-igb",
                "display_name": "Intel I210/I211/I350/82575/82576 (igb)",
                "description": "Intel Gigabit Ethernet driver for server-class NICs. Supports 82575/82576/82580/I210/I211/I350/I354 controllers with SR-IOV.",
                "category": "network",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x8086", "device": "0x1521"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x1533"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x1536"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x150E"},
                ],
                "min_kernel": "2.6.24",
                "kernel_path": "drivers/net/ethernet/intel/igb",
                "vendor": "Intel",
                "chipset_family": "Intel I210/I350",
                "dependencies": [],
                "tags": ["network", "ethernet", "intel", "server", "sriov"],
                "license": "GPL-2.0-only",
                "maintainer": "Intel Wired Networking",
            },
            # ── Wireless ─────────────────────────────────────────────
            {
                "id": "linux-iwlwifi",
                "display_name": "Intel Wi-Fi 6/6E/7 (iwlwifi)",
                "description": "Intel Wireless driver supporting Wi-Fi 6 (AX200/AX201), Wi-Fi 6E (AX210/AX211), and Wi-Fi 7 (BE200). Requires firmware blobs.",
                "category": "wireless",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x8086", "device": "0x2723"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x2725"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x272B"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x7A70"},
                ],
                "min_kernel": "3.2.0",
                "kernel_path": "drivers/net/wireless/intel/iwlwifi",
                "vendor": "Intel",
                "chipset_family": "Intel AX200/AX210/BE200",
                "dependencies": [
                    {"name": "iwlwifi-ty-a0-gf-a0-89.ucode", "kind": "firmware",
                     "source": "linux-firmware", "required": True},
                    {"name": "iwlwifi-so-a0-hr-b0-83.ucode", "kind": "firmware",
                     "source": "linux-firmware", "required": True},
                ],
                "tags": ["wireless", "wifi", "intel", "wifi6", "wifi6e", "wifi7", "ax200", "ax210"],
                "license": "GPL-2.0-only",
                "maintainer": "Intel Linux Wireless <linuxwifi@intel.com>",
            },
            {
                "id": "linux-ath9k",
                "display_name": "Qualcomm Atheros AR9xxx (ath9k)",
                "description": "Open-source Atheros Wi-Fi driver for AR9220/AR9280/AR9285/AR9287/AR9380/AR9462/AR9485/AR9565. Fully open — no firmware blob required.",
                "category": "wireless",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x168C", "device": "0x0029"},
                    {"bus": "pci", "vendor": "0x168C", "device": "0x002A"},
                    {"bus": "pci", "vendor": "0x168C", "device": "0x002B"},
                    {"bus": "pci", "vendor": "0x168C", "device": "0x0030"},
                    {"bus": "pci", "vendor": "0x168C", "device": "0x0032"},
                    {"bus": "pci", "vendor": "0x168C", "device": "0x0036"},
                ],
                "min_kernel": "2.6.27",
                "kernel_path": "drivers/net/wireless/ath/ath9k",
                "vendor": "Qualcomm Atheros",
                "chipset_family": "AR9285/AR9462",
                "dependencies": [],
                "tags": ["wireless", "wifi", "atheros", "qualcomm", "open-source", "no-firmware"],
                "license": "ISC",
                "maintainer": "ath9k-devel@qca.qualcomm.com",
            },
            {
                "id": "linux-ath11k",
                "display_name": "Qualcomm Wi-Fi 6/6E (ath11k)",
                "description": "Qualcomm Wi-Fi 6 driver for QCA6390/WCN6855/QCN9074/IPQ8074. Supports Wi-Fi 6E tri-band operation.",
                "category": "wireless",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x17CB", "device": "0x1103"},
                    {"bus": "pci", "vendor": "0x17CB", "device": "0x1101"},
                ],
                "min_kernel": "5.6.0",
                "kernel_path": "drivers/net/wireless/ath/ath11k",
                "vendor": "Qualcomm",
                "chipset_family": "QCA6390/WCN6855",
                "dependencies": [
                    {"name": "ath11k/WCN6855/hw2.1/", "kind": "firmware",
                     "source": "linux-firmware", "required": True}
                ],
                "tags": ["wireless", "wifi", "qualcomm", "wifi6", "wifi6e"],
                "license": "GPL-2.0-only",
                "maintainer": "ath11k@lists.infradead.org",
            },
            {
                "id": "linux-mt76",
                "display_name": "MediaTek MT76xx Wi-Fi (mt76)",
                "description": "MediaTek Wi-Fi driver for MT7610/MT7612/MT7615/MT7622/MT7663/MT7915/MT7921/MT7922/MT7925. Budget and mid-range Wi-Fi 6/6E.",
                "category": "wireless",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x14C3", "device": "0x7961"},
                    {"bus": "pci", "vendor": "0x14C3", "device": "0x7922"},
                    {"bus": "pci", "vendor": "0x14C3", "device": "0x7925"},
                    {"bus": "usb", "vendor": "0x0E8D", "product": "0x7961"},
                ],
                "min_kernel": "4.18.0",
                "kernel_path": "drivers/net/wireless/mediatek/mt76",
                "vendor": "MediaTek",
                "chipset_family": "MT7921/MT7922",
                "dependencies": [
                    {"name": "mediatek/WIFI_MT7961_patch_mcu_1_2_hdr.bin", "kind": "firmware",
                     "source": "linux-firmware", "required": True}
                ],
                "tags": ["wireless", "wifi", "mediatek", "wifi6", "budget"],
                "license": "ISC",
                "maintainer": "Felix Fietkau <nbd@nbd.name>",
            },
            # ── Storage ──────────────────────────────────────────────
            {
                "id": "linux-nvme",
                "display_name": "NVM Express (nvme)",
                "description": "NVMe SSD driver. Supports all NVMe 1.x/2.x compliant drives via PCI Express. High-performance direct-to-hardware I/O.",
                "category": "storage",
                "status": "active",
                "compat_status": "native",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x8086", "device": "0xF1A8", "class": "0x010802"},
                    {"bus": "pci", "vendor": "0x144D", "device": "0xA808", "class": "0x010802"},
                    {"bus": "pci", "vendor": "0x144D", "device": "0xA809", "class": "0x010802"},
                ],
                "min_kernel": "3.3.0",
                "kernel_path": "drivers/nvme/host",
                "vendor": "NVM Express Consortium",
                "chipset_family": "NVMe generic",
                "sigma_module": "sigma-nvme",
                "dependencies": [],
                "tags": ["storage", "nvme", "ssd", "pcie", "high-performance"],
                "license": "GPL-2.0-only",
                "maintainer": "Keith Busch <kbusch@kernel.org>",
            },
            {
                "id": "linux-ahci",
                "display_name": "AHCI SATA Controller (ahci)",
                "description": "Advanced Host Controller Interface driver for SATA. Supports hot-plug, NCQ, port multiplier. Standard for all modern SATA controllers.",
                "category": "storage",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x8086", "device": "0x2922"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0xA352"},
                    {"bus": "pci", "vendor": "0x1002", "device": "0x4390"},
                    {"bus": "pci", "vendor": "0x1002", "device": "0x4391"},
                ],
                "min_kernel": "2.6.19",
                "kernel_path": "drivers/ata",
                "vendor": "Generic",
                "chipset_family": "AHCI",
                "dependencies": [],
                "tags": ["storage", "sata", "ahci", "hdd", "ssd"],
                "license": "GPL-2.0-only",
                "maintainer": "linux-ide@vger.kernel.org",
            },
            # ── GPU ──────────────────────────────────────────────────
            {
                "id": "linux-i915",
                "display_name": "Intel HD/UHD/Iris/Arc Graphics (i915)",
                "description": "Intel integrated and discrete GPU driver. Supports Gen4 through Gen12+ (Haswell, Skylake, Tiger Lake, Alder Lake, Arc). KMS, GEM, GuC/HuC firmware.",
                "category": "gpu",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x8086", "device": "0x9A49"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x4680"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x46A6"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x5690"},
                ],
                "min_kernel": "2.6.28",
                "kernel_path": "drivers/gpu/drm/i915",
                "vendor": "Intel",
                "chipset_family": "Intel Gen9-Gen13",
                "dependencies": [
                    {"name": "i915/tgl_guc_70.bin", "kind": "firmware",
                     "source": "linux-firmware", "required": False},
                    {"name": "i915/tgl_huc.bin", "kind": "firmware",
                     "source": "linux-firmware", "required": False},
                ],
                "tags": ["gpu", "graphics", "intel", "kms", "vulkan", "opengl"],
                "license": "GPL-2.0-only AND MIT",
                "maintainer": "Intel Graphics <intel-gfx@lists.freedesktop.org>",
            },
            {
                "id": "linux-amdgpu",
                "display_name": "AMD Radeon Graphics (amdgpu)",
                "description": "AMD GPU driver for Radeon RX 400+ (Polaris, Vega, RDNA 1/2/3). Open-source kernel driver with Mesa userspace. Supports Vulkan, OpenGL, compute.",
                "category": "gpu",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x1002", "device": "0x67DF"},
                    {"bus": "pci", "vendor": "0x1002", "device": "0x731F"},
                    {"bus": "pci", "vendor": "0x1002", "device": "0x73BF"},
                    {"bus": "pci", "vendor": "0x1002", "device": "0x744C"},
                ],
                "min_kernel": "4.2.0",
                "kernel_path": "drivers/gpu/drm/amd/amdgpu",
                "vendor": "AMD",
                "chipset_family": "Radeon RX 400-7000",
                "dependencies": [
                    {"name": "amdgpu/navi10_gpu_info.bin", "kind": "firmware",
                     "source": "linux-firmware", "required": True}
                ],
                "tags": ["gpu", "graphics", "amd", "radeon", "vulkan", "mesa", "compute"],
                "license": "GPL-2.0-only AND MIT",
                "maintainer": "amd-gfx@lists.freedesktop.org",
            },
            {
                "id": "linux-nouveau",
                "display_name": "NVIDIA Open GPU (nouveau)",
                "description": "Community open-source NVIDIA GPU driver. Supports GeForce 6 through Turing (limited). No firmware needed for pre-Maxwell. Limited reclocking.",
                "category": "gpu",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x10DE", "device": "0x1B80"},
                    {"bus": "pci", "vendor": "0x10DE", "device": "0x1E04"},
                    {"bus": "pci", "vendor": "0x10DE", "device": "0x2204"},
                ],
                "min_kernel": "2.6.33",
                "kernel_path": "drivers/gpu/drm/nouveau",
                "vendor": "NVIDIA (Community)",
                "chipset_family": "GeForce GTX/RTX",
                "dependencies": [],
                "tags": ["gpu", "graphics", "nvidia", "nouveau", "open-source"],
                "license": "GPL-2.0-only AND MIT",
                "maintainer": "nouveau@lists.freedesktop.org",
            },
            # ── Audio ────────────────────────────────────────────────
            {
                "id": "linux-snd-hda-intel",
                "display_name": "Intel HD Audio (snd-hda-intel)",
                "description": "Intel High Definition Audio controller driver. Standard audio on all modern Intel and AMD motherboards. Supports HDA codecs: Realtek, Conexant, IDT, Cirrus Logic.",
                "category": "audio",
                "status": "active",
                "compat_status": "untested",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x8086", "device": "0xA0C8"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x43C8"},
                    {"bus": "pci", "vendor": "0x1002", "device": "0xAB38"},
                ],
                "min_kernel": "2.6.13",
                "kernel_path": "sound/pci/hda",
                "vendor": "Intel/Generic",
                "chipset_family": "HDA Controller",
                "dependencies": [],
                "tags": ["audio", "sound", "hda", "intel", "realtek"],
                "license": "GPL-2.0-or-later",
                "maintainer": "alsa-devel@alsa-project.org",
            },
            # ── Input ────────────────────────────────────────────────
            {
                "id": "linux-xhci-hcd",
                "display_name": "USB 3.x Host Controller (xhci-hcd)",
                "description": "eXtensible Host Controller Interface driver for USB 3.0/3.1/3.2 controllers. Universal driver for all USB 3.x ports.",
                "category": "usb",
                "status": "active",
                "compat_status": "native",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x8086", "device": "0xA36D", "class": "0x0C0330"},
                    {"bus": "pci", "vendor": "0x8086", "device": "0x43ED", "class": "0x0C0330"},
                    {"bus": "pci", "vendor": "0x1022", "device": "0x149C", "class": "0x0C0330"},
                ],
                "min_kernel": "2.6.31",
                "kernel_path": "drivers/usb/host",
                "vendor": "USB-IF",
                "chipset_family": "xHCI generic",
                "sigma_module": "sigma-xhci",
                "dependencies": [],
                "tags": ["usb", "usb3", "xhci", "host-controller"],
                "license": "GPL-2.0-only",
                "maintainer": "linux-usb@vger.kernel.org",
            },
            # ── VirtIO ───────────────────────────────────────────────
            {
                "id": "linux-virtio-net",
                "display_name": "VirtIO Network Device (virtio-net)",
                "description": "Paravirtualized network driver for KVM/QEMU virtual machines. High-performance with vhost acceleration.",
                "category": "virtio",
                "status": "active",
                "compat_status": "native",
                "hardware_ids": [
                    {"bus": "virtio", "device_id": 1, "vendor_id": 0x1AF4},
                ],
                "min_kernel": "2.6.25",
                "kernel_path": "drivers/net/virtio_net.c",
                "vendor": "Red Hat",
                "chipset_family": "VirtIO",
                "sigma_module": "sigma-virtio-net",
                "dependencies": [],
                "tags": ["virtio", "network", "kvm", "qemu", "virtual"],
                "license": "GPL-2.0-only",
                "maintainer": "virtualization@lists.linux.dev",
            },
            {
                "id": "linux-virtio-blk",
                "display_name": "VirtIO Block Device (virtio-blk)",
                "description": "Paravirtualized block device driver for KVM/QEMU. Provides high-performance virtual disk I/O.",
                "category": "virtio",
                "status": "active",
                "compat_status": "native",
                "hardware_ids": [
                    {"bus": "virtio", "device_id": 2, "vendor_id": 0x1AF4},
                ],
                "min_kernel": "2.6.25",
                "kernel_path": "drivers/block/virtio_blk.c",
                "vendor": "Red Hat",
                "chipset_family": "VirtIO",
                "sigma_module": "sigma-virtio-blk",
                "dependencies": [],
                "tags": ["virtio", "storage", "block", "kvm", "qemu", "virtual"],
                "license": "GPL-2.0-only",
                "maintainer": "virtualization@lists.linux.dev",
            },
            # ── Deprecated / Removed Drivers ─────────────────────────
            {
                "id": "linux-3c59x",
                "display_name": "3Com Vortex/Boomerang (3c59x)",
                "description": "3Com EtherLink PCI III/XL, Hurricane, Tornado series. Legacy but still widely used in older systems and QEMU emulation.",
                "category": "network",
                "status": "deprecated",
                "compat_status": "shimmed",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x10B7", "device": "0x5900"},
                    {"bus": "pci", "vendor": "0x10B7", "device": "0x5950"},
                    {"bus": "pci", "vendor": "0x10B7", "device": "0x5951"},
                    {"bus": "pci", "vendor": "0x10B7", "device": "0x9200"},
                ],
                "min_kernel": "2.0.0",
                "kernel_path": "drivers/net/ethernet/3com/3c59x.c",
                "vendor": "3Com",
                "chipset_family": "3Com Vortex 3c905",
                "dependencies": [],
                "tags": ["network", "ethernet", "3com", "legacy", "vortex", "qemu"],
                "license": "GPL-2.0-only",
                "maintainer": "",
            },
            {
                "id": "linux-tulip",
                "display_name": "DEC Tulip (tulip)",
                "description": "Digital Equipment Corporation / Intel 21140/21142/21143 Ethernet driver. Classic 100Mbit NIC, extremely popular in the 1990s.",
                "category": "network",
                "status": "deprecated",
                "compat_status": "shimmed",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x1011", "device": "0x0009"},
                    {"bus": "pci", "vendor": "0x1011", "device": "0x0019"},
                    {"bus": "pci", "vendor": "0x1317", "device": "0x0985"},
                ],
                "min_kernel": "2.0.0",
                "kernel_path": "drivers/net/ethernet/dec/tulip",
                "vendor": "DEC/Intel",
                "chipset_family": "DEC 21140/21143",
                "dependencies": [],
                "tags": ["network", "ethernet", "dec", "tulip", "legacy", "vintage"],
                "license": "GPL-2.0-only",
                "maintainer": "",
            },
            {
                "id": "linux-rtl8139",
                "display_name": "Realtek RTL8139 (8139too)",
                "description": "Realtek RTL8139/RTL8130 10/100 Ethernet driver. Extremely common in budget PCs and virtual machines. QEMU default NIC.",
                "category": "network",
                "status": "deprecated",
                "compat_status": "shimmed",
                "hardware_ids": [
                    {"bus": "pci", "vendor": "0x10EC", "device": "0x8139"},
                    {"bus": "pci", "vendor": "0x10EC", "device": "0x8138"},
                ],
                "min_kernel": "2.4.0",
                "kernel_path": "drivers/net/ethernet/realtek/8139too.c",
                "vendor": "Realtek",
                "chipset_family": "RTL8139",
                "dependencies": [],
                "tags": ["network", "ethernet", "realtek", "legacy", "qemu", "100mbit"],
                "license": "GPL-2.0-only",
                "maintainer": "",
            },
            {
                "id": "linux-vesafb",
                "display_name": "VESA Framebuffer (vesafb)",
                "description": "Generic VESA VBE framebuffer driver. Universal fallback when no GPU-specific driver is available. Basic 2D only, no acceleration.",
                "category": "display",
                "status": "active",
                "compat_status": "shimmed",
                "hardware_ids": [],
                "min_kernel": "2.2.0",
                "kernel_path": "drivers/video/fbdev/vesafb.c",
                "vendor": "VESA Consortium",
                "chipset_family": "VESA VBE generic",
                "dependencies": [],
                "tags": ["display", "framebuffer", "vesa", "fallback", "universal"],
                "license": "GPL-2.0-only",
                "maintainer": "",
            },
            {
                "id": "linux-usb-storage",
                "display_name": "USB Mass Storage (usb-storage)",
                "description": "USB Mass Storage class driver. Supports virtually all USB flash drives, external HDDs, card readers, and USB optical drives.",
                "category": "usb",
                "status": "active",
                "compat_status": "shimmed",
                "hardware_ids": [],
                "min_kernel": "2.4.0",
                "kernel_path": "drivers/usb/storage",
                "vendor": "USB-IF",
                "chipset_family": "USB MSC generic",
                "dependencies": [],
                "tags": ["usb", "storage", "mass-storage", "flash-drive", "universal"],
                "license": "GPL-2.0-only",
                "maintainer": "linux-usb@vger.kernel.org",
            },
        ],
    }


def main():
    parser = argparse.ArgumentParser(
        description="SigmaOS Driver Catalogue Builder — auto-generates driver_catalogue.json"
    )
    parser.add_argument(
        "--linux-tree", type=str, default=None,
        help="Path to a local Linux kernel git tree (e.g., /path/to/linux)"
    )
    parser.add_argument(
        "--output", "-o", type=str, default="data/driver_catalogue.json",
        help="Output path for the generated catalogue JSON"
    )
    parser.add_argument(
        "--test-mode", action="store_true",
        help="Generate a seed catalogue from built-in data (no kernel tree needed)"
    )
    parser.add_argument(
        "--merge", type=str, default=None,
        help="Merge results with an existing catalogue JSON"
    )

    args = parser.parse_args()

    if args.test_mode or args.linux_tree is None:
        print("═" * 60)
        print("  SigmaOS Driver Catalogue Builder — Seed Mode")
        print("═" * 60)
        catalogue = generate_seed_catalogue()
    else:
        print("═" * 60)
        print(f"  SigmaOS Driver Catalogue Builder")
        print(f"  Scanning: {args.linux_tree}")
        print("═" * 60)
        drivers = scan_linux_tree(args.linux_tree)
        catalogue = {
            "version": "1.0.0",
            "generated": "2026-07-09T00:00:00Z",
            "source": args.linux_tree,
            "drivers": drivers,
        }

    # Merge with existing if requested
    if args.merge and os.path.exists(args.merge):
        print(f"Merging with existing catalogue: {args.merge}")
        with open(args.merge, "r") as f:
            existing = json.load(f)
        existing_ids = {d["id"] for d in existing.get("drivers", [])}
        new_drivers = [d for d in catalogue["drivers"] if d["id"] not in existing_ids]
        existing["drivers"].extend(new_drivers)
        catalogue = existing
        print(f"  Added {len(new_drivers)} new drivers")

    # Write output
    os.makedirs(os.path.dirname(args.output) or ".", exist_ok=True)
    with open(args.output, "w") as f:
        json.dump(catalogue, f, indent=2, ensure_ascii=False)

    print(f"\n✓ Catalogue written to {args.output}")
    print(f"  Total drivers: {len(catalogue['drivers'])}")

    # Print summary by category
    categories = {}
    for d in catalogue["drivers"]:
        cat = d.get("category", "misc")
        categories[cat] = categories.get(cat, 0) + 1

    print("\n  By category:")
    for cat, count in sorted(categories.items(), key=lambda x: -x[1]):
        print(f"    {cat:<15} {count}")


if __name__ == "__main__":
    main()
