#!/bin/bash
# SigmaOS firewall configuration (install/config/firewall.sh)

set -euo pipefail

if command -v nft >/dev/null 2>&1; then
    nft add table inet filter 2>/dev/null || true
    nft add chain inet filter input '{ type filter hook input priority 0; policy drop; }' 2>/dev/null || true
    nft add rule inet filter input ct state established,related accept 2>/dev/null || true
    nft add rule inet filter input iifname "lo" accept 2>/dev/null || true
    nft add rule inet filter input icmp type echo-request accept 2>/dev/null || true
    nft add rule inet filter input udp dport 67 accept 2>/dev/null || true   # DHCP
    nft add rule inet filter input tcp dport 22 accept 2>/dev/null || true   # SSH
    nft add rule inet filter input tcp dport 80 accept 2>/dev/null || true   # HTTP
    nft add rule inet filter input tcp dport 443 accept 2>/dev/null || true  # HTTPS
    echo "[sigmaos-firewall] Firewall configured"
else
    echo "[sigmaos-firewall] nft not available, skipping firewall"
fi
