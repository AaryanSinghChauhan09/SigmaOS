# sigma-net Manual

## NAME

`sigma-net` — SigmaOS network management CLI

## SYNOPSIS

```
sigma-net <command> [options]
sigma-net --version
sigma-net --help
```

## DESCRIPTION

`sigma-net` is the primary network management interface for SigmaOS. It manages network interfaces, routing tables, DNS, WiFi connections, firewall rules, and runs diagnostics. On bare metal it delegates to the SigmaOS sovereign TCP/IP stack via `/run/sigma/netd.sock`. Source: `tools/sigma-net.rs`.

## INTERFACE COMMANDS

### `status [iface]`

Show all network interfaces or one specific:
```bash
sigma-net status            # all interfaces

sigma-net status eth0       # one interface

sigma-net status --json     # JSON for scripting

```

### `up / down <iface>`

```bash
sigma-net up eth0
sigma-net down wlan0
```

### `ip <iface> <addr/prefix>`

Set a static IP address:
```bash
sigma-net ip eth0 10.0.0.10/24
sigma-net ip eth0 192.168.1.100/24
```

### `dhcp <iface>`

Request a DHCP lease (delegates to `dhclient` or `udhcpc`):
```bash
sigma-net dhcp eth0
sigma-net dhcp wlan0
```

### `mac <iface> [new-mac]`

Show or set MAC address:
```bash
sigma-net mac eth0                        # show MAC

sigma-net mac eth0 52:54:00:ab:cd:ef     # set MAC (requires root)

```

### `stats [iface]`

Interface statistics (RX/TX bytes, packets, errors, drops):
```bash
sigma-net stats
sigma-net stats eth0 --json
```

## ROUTING

```bash
sigma-net route list                          # show routing table

sigma-net route add 192.168.1.0/24 via 10.0.0.1   # add route

sigma-net route del 192.168.1.0/24           # remove route

```

## DNS

```bash
sigma-net dns show                  # show /etc/resolv.conf

sigma-net dns set 1.1.1.1           # set DNS resolver

sigma-net dns resolve sigmaos.app   # lookup hostname

```

## DIAGNOSTICS

### `ping <host> [-c n]`

ICMP ping (delegates to system ping; simulated when unavailable):
```bash
sigma-net ping 8.8.8.8
sigma-net ping sigmaos.app -c 10
```

### `trace <host>`

Traceroute (delegates to `traceroute`):
```bash
sigma-net trace 8.8.8.8
```

### `scan <subnet>`

ARP network discovery:
```bash
sigma-net scan 10.0.0.0/24      # discover hosts

sigma-net scan 192.168.1.0/24 --json
```

### `capture <iface> [-n count]`

Packet capture (delegates to `tcpdump`; writes `/tmp/sigma-cap.pcap`):
```bash
sigma-net capture eth0 -n 32
```

## WIFI

```bash
sigma-net wifi scan                          # scan for networks

sigma-net wifi connect "MyNetwork" "psk"    # connect WPA3

sigma-net wifi disconnect
sigma-net wifi status
sigma-net wifi status --json
```

WiFi scan output shows RSSI signal bars and security type (WPA3/WPA2/Open).

## FIREWALL

```bash
sigma-net fw list                       # show rules

sigma-net fw allow "tcp dport 8080"     # add allow rule

sigma-net fw deny "tcp dport 23"        # add deny rule

sigma-net fw flush --force              # remove all rules

```

## OPTIONS

| Flag | Description |
|------|-------------|
| `-c`, `--count <n>` | Ping packet count (default: 4) |
| `-n`, `--num <n>` | Capture packet count (default: 16) |
| `--force` | Override safety prompts (required for `fw flush`) |
| `--json` | Machine-readable JSON output |
| `--version`, `-V` | Print version |
| `--help`, `-h` | Show help |

## EXAMPLES

```bash

# Full network status

sigma-net status --json | jq '.[].addr'

# Set up a new interface

sigma-net up eth1
sigma-net ip eth1 10.0.1.1/24

# WiFi workflow

sigma-net wifi scan
sigma-net wifi connect "SigmaNet" "mypassphrase"
sigma-net wifi status

# Diagnose connectivity

sigma-net ping 8.8.8.8 -c 5
sigma-net trace sigmaos.app
sigma-net scan 10.0.0.0/24

# Firewall rules

sigma-net fw list
sigma-net fw allow "tcp dport 443"
sigma-net fw deny "tcp dport 3389"

# Route management

sigma-net route list --json
sigma-net route add 10.10.0.0/16 via 10.0.0.254
```

## VERSION

sigma-net 1.0.0

## SEE ALSO

`sigma-cli net(1)`, `sigma_diagnostics(1)`, `sigma-secure(1)`, `sigma_fsck(1)`
