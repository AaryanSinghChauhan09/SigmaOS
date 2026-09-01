# 💿 Installation & Deployment Guide

SigmaOS provides multiple installation options: Calamares-style Graphical Installer, Live ISO media, Windows dual-boot loopback installer, and Unified Kernel Image (UKI) direct boot.

---

## 1. Graphical Web Installer (`web_ui/index.html` & `web_ui/styles/style.css`)

* **Calamares-Style Wizard:** Multi-step wizard guiding users through Language Selection, Disk Partitioning, User Account Setup, and Installation Progress.
* **Accessibility Features:** Heading focus management (`tabindex="-1"` with `.focus()`), WAI-ARIA radio group arrow key navigation, and real-time validation error alerts.
* **Verified by CI:** Verified for Web OS integrity and CSS style rules via `.github/workflows/web.yml`.

---

## 2. Windows Dual-Boot Loopback Installer (`src/compatibility/mint_linux.rs` & `src/tools/installer.rs`)

* **mint4win & Wubi Inspired:** Installs SigmaOS inside a virtual loopback disk file on an existing NTFS Windows partition (`SigmaOS.disk`).
* **BCD Bootloader Integration:** Automatically registers a Windows BCD boot entry (`bcdedit /create ... /d "SigmaOS (Linux Mint Dual-Boot)"`).

---

## 3. Unified Kernel Image (UKI) & EFISTUB Boot (`src/boot/sigma_boot.rs`)

* **UKI PE/COFF Headers:** Direct booting via `.linux`, `.initrd`, `.cmdline`, `.osrel`, and `.splash` sections wrapped in a single Signed EFI binary.
* **EFISTUB Boot Manager:** Directly booted by UEFI firmware without requiring an intermediate bootloader.
