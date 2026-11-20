#!/usr/bin/env bash
# SigmaOS Guide — Troubleshooting Guide CLI scaffold
# Upstream: Arch Wiki (https://wiki.archlinux.org/title/General_troubleshooting)
set -euo pipefail

GUIDE_VERSION="0.5"
GUIDE_NAME="SigmaOS Troubleshooting Guide"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand>

$GUIDE_NAME v$GUIDE_VERSION

Subcommands:
  boot        Boot failures and recovery
  display     Display and GPU issues
  network     Network connectivity problems
  audio       Audio not working
  packages    Package manager errors
  desktop     Desktop and compositor issues
  hardware    Hardware not detected
  logs        How to collect diagnostic logs
  recovery    Emergency recovery procedures
  list        List all topics
  version     Print guide version

EOF
}

topic_boot() { cat <<'EOF'
Boot Troubleshooting
=====================
Symptom: Black screen after GRUB
  → Try nomodeset in kernel parameters (edit GRUB entry, add to linux line)
  → Check GPU driver: sigmaos-drivers --diagnostics

Symptom: "Failed to mount /boot/efi"
  → Verify EFI partition: lsblk; mount /dev/sdX1 /boot/efi

Symptom: Boot loop (reboots 3×)
  → SigmaOS-boot auto-selects previous kernel after 3 failures
  → Boot previous kernel entry in GRUB and file a bug

Symptom: initramfs drops to emergency shell
  → Check cmdline: cat /proc/cmdline
  → Verify root UUID: blkid /dev/sdX3
  → Edit /etc/fstab UUID from live USB chroot

Logs to collect: /var/log/sigmaos/boot.log  +  serial console
EOF
}

topic_display() { cat <<'EOF'
Display / GPU Troubleshooting
==============================
No display after login:
  1. Switch to TTY: Ctrl+Alt+F2
  2. Check compositor: journalctl --user -u sigmaos-desktop
  3. Verify GPU driver: sigmaos-drivers --diagnostics
  4. Fallback to software rendering: LIBGL_ALWAYS_SOFTWARE=1 sway

Wrong resolution:
  → Settings → Display → select correct resolution
  → CLI: wlr-randr --output <name> --mode <WxH>

Tearing / artifacts:
  → Check vsync: in compositor config set  vsync = on
  → For NVIDIA: ensure proprietary driver is loaded (not nouveau)

Multiple monitors not detected:
  → wlr-randr --output <name> --on
  → sigmaos-hardware --redetect
EOF
}

topic_network() { cat <<'EOF'
Network Troubleshooting
========================
No internet after boot:
  → ip link  — are interfaces up?
  → nmcli device status
  → systemctl status NetworkManager

Wi-Fi not connecting:
  → nmcli device wifi list  — is SSID visible?
  → nmcli device wifi connect <SSID> password <pass>
  → Check firmware: sigmaos-drivers --diagnostics | grep wifi

DNS not resolving:
  → cat /etc/resolv.conf  — should show nameserver entries
  → systemd-resolve --status
  → Flush: resolvectl flush-caches

Firewall blocking traffic:
  → nft list ruleset
  → sigmaos-network --sync-firewall
EOF
}

topic_audio() { cat <<'EOF'
Audio Troubleshooting
======================
No sound:
  1. Check PipeWire: systemctl --user status pipewire
  2. wpctl status  — list audio devices
  3. Unmute: wpctl set-mute @DEFAULT_AUDIO_SINK@ 0
  4. Set volume: wpctl set-volume @DEFAULT_AUDIO_SINK@ 0.8

Wrong output device:
  → wpctl set-default <sink-id>  (get ID from wpctl status)
  → Settings → Sound → select output device

Microphone not working:
  → wpctl status | grep Source
  → wpctl set-mute @DEFAULT_AUDIO_SOURCE@ 0

Bluetooth audio:
  → blueman-manager or Settings → Bluetooth
  → Reconnect device if codec changed after update
EOF
}

topic_packages() { cat <<'EOF'
Package Manager Troubleshooting
=================================
Package install fails:
  → sigmaos-repo --sync --force  (refresh metadata)
  → sigmaos-pkg --self-check     (database integrity)
  → Check space: df -h /

Transaction locked:
  → sigmaos-pkg --clear-lock     (safe if no process holds it)

Signature verification failed:
  → sigmaos-security --refresh-trust
  → Re-sync repo metadata

Dependency conflict:
  → Read the conflict report; use --assume-installed to override a dep
  → File a bug if two SigmaOS packages conflict

Broken package database:
  → sigmaos-pkg --rebuild-db
EOF
}

topic_desktop() { cat <<'EOF'
Desktop Troubleshooting
========================
Desktop doesn't start:
  → journalctl --user -u sigmaos-desktop -n 50
  → Check compositor: sway --validate

Panel missing:
  → sigmaos-desktop-session --restart-panel
  → Check Waybar config: waybar --log-level debug

Launcher won't open (Super+Space):
  → sigmaos-launcher --reindex
  → Check keybind: grep -r launcher ~/.config/sigmaos/

Theme broken / fallback theme:
  → sigmaos-theme --validate-active
  → sigmaos-theme --switch sigmaos-light  (reset to default)

App not in launcher:
  → sigmaos-launcher --reindex
  → Verify .desktop file: desktop-file-validate /usr/share/applications/<app>.desktop
EOF
}

topic_hardware() { cat <<'EOF'
Hardware Not Detected
======================
Device missing:
  → sigmaos-hardware --redetect
  → sigmaos-drivers --diagnostics
  → Check kernel log: dmesg | grep -i error | tail -30

Printer not found:
  → systemctl status cups
  → lpinfo -v  (list detected printers)

Bluetooth adapter missing:
  → rfkill list bluetooth  (check if soft-blocked)
  → rfkill unblock bluetooth

USB device not recognized:
  → dmesg | grep -i usb | tail -20
  → lsusb  (compare against known-supported list)
EOF
}

topic_logs() { cat <<'EOF'
Collecting Diagnostic Logs
============================
Boot logs:
  journalctl -b -0              # current boot
  journalctl -b -1              # previous boot
  /var/log/sigmaos/boot.log

Service logs:
  journalctl -u <service>       # system service
  journalctl --user -u <svc>   # user service

Hardware/driver:
  sigmaos-hardware --diagnostics
  sigmaos-drivers  --diagnostics
  dmesg | tail -100

Package manager:
  /var/log/sigmaos/pkg.log

Network:
  journalctl -u NetworkManager

Attach ALL logs when filing a bug report.
EOF
}

topic_recovery() { cat <<'EOF'
Emergency Recovery
===================
Boot to recovery mode:
  → At GRUB, select "SigmaOS Recovery" entry
  → Or: add  recovery  to kernel cmdline

Chroot from live USB:
  mount /dev/sdX3 /mnt
  mount /dev/sdX1 /mnt/boot/efi
  arch-chroot /mnt

Reset root password:
  → In chroot: passwd root

Reinstall bootloader:
  → bootctl install --esp-path=/boot/efi
  → sigmaos-boot-entry --regenerate

Rollback last update:
  → sigmaos-update --rollback
  → Or boot previous kernel from GRUB

Factory reset (preserves /home):
  → sigmaos-installer install --keep-home --disk /dev/sdX
EOF
}

list_topics() {
    echo "Troubleshooting topics:"
    for t in boot display network audio packages desktop hardware logs recovery; do
        echo "  $t"
    done
}

case "${1:-}" in
    boot)        topic_boot ;;
    display)     topic_display ;;
    network)     topic_network ;;
    audio)       topic_audio ;;
    packages)    topic_packages ;;
    desktop)     topic_desktop ;;
    hardware)    topic_hardware ;;
    logs)        topic_logs ;;
    recovery)    topic_recovery ;;
    list)        list_topics ;;
    version)     echo "$GUIDE_NAME v$GUIDE_VERSION" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown topic: $1" >&2; usage; exit 1 ;;
esac
