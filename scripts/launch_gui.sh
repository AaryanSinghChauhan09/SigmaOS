#!/usr/bin/env bash
# SigmaOS Graphical Desktop Launcher
# Principles: LEAST ASTONISHMENT, READY TO USE, DISCOVERABILITY

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

if ! command -v zenity >/dev/null 2>&1; then
    echo "zenity is not installed. Running terminal workstation instead..."
    exec "$ROOT_DIR/bin/sigmaos"
fi

CHOICE=$(zenity --list \
    --title="SigmaOS Sovereign Workstation" \
    --text="Select an action to launch:" \
    --column="Action" --column="Description" \
    --width=680 --height=420 \
    "Omarchy" "Launch Omarchy Aesthetic Persona (Hyprland / Wallust)" \
    "Mint" "Launch Linux Mint Persona (Cinnamon / XApps)" \
    "Arch" "Launch Arch Linux Persona (Pacman / Rolling)" \
    "Boot-QEMU" "Boot SigmaOS Stub ISO image in QEMU virtual machine" \
    "Craft-Suite" "View 20-Module Craft Creative Suite (3D, Art, CAD, ...)" \
    "Doctor" "Execute Omarchy system diagnostics health check" \
    "Readiness-Check" "Execute 21-point automated launch readiness validation" \
    "Status" "Display kernel, package, and subsystem status" \
    2>/dev/null || echo "")

case "$CHOICE" in
    "Omarchy")
        bash "$ROOT_DIR/scripts/launch_omarchy_persona.sh"
        ;;
    "Mint")
        bash "$ROOT_DIR/scripts/launch_mint_persona.sh"
        ;;
    "Arch")
        bash "$ROOT_DIR/scripts/launch_arch_persona.sh"
        ;;
    "Boot-QEMU")
        if command -v qemu-system-x86_64 >/dev/null 2>&1; then
            exec qemu-system-x86_64 -m 2048 -cdrom "$ROOT_DIR/build/sigmaos-desktop-preview.iso" -serial stdio -no-reboot
        else
            zenity --warning --title="QEMU Not Found" \
                --text="qemu-system-x86_64 is not installed.\n\nTo install QEMU, run in your terminal:\nsudo apt install -y qemu-system-x86 nasm xorriso" \
                --width=450 2>/dev/null || true
        fi
        ;;
    "Craft-Suite")
        "$ROOT_DIR/bin/sigmaos" --craft | zenity --text-info --title="20-Module Craft Creative Suite" --width=600 --height=450 2>/dev/null || true
        ;;
    "Doctor")
        "$ROOT_DIR/bin/sigmaos" --doctor | zenity --text-info --title="SigmaOS System Doctor" --width=600 --height=450 2>/dev/null || true
        ;;
    "Readiness-Check")
        "$ROOT_DIR/bin/sigmaos" --check | zenity --text-info --title="Launch Readiness Validation" --width=600 --height=450 2>/dev/null || true
        ;;
    "Status")
        "$ROOT_DIR/bin/sigmaos" --status | zenity --text-info --title="SigmaOS Launch Status" --width=600 --height=450 2>/dev/null || true
        ;;
    *)
        exit 0
        ;;
esac
