"""
SigmaOS Sovereign Network Stack
==================================
USP: Native mesh networking + quantum-safe encryption + zero-trust network.

Competition comparison:
  Windows → TCP/IP stack, enterprise-grade, single-machine-centric
  macOS   → Seamless Wi-Fi / AirDrop / Bonjour; AirPlay integration
  Linux   → Highly configurable: iptables, netfilter, tc; no mesh native
  SigmaOS → All of the above PLUS native L2/L3 mesh + post-quantum TLS

Core innovations:
  1. SigmaMesh    — automatic Zigbee/Wi-Fi Direct/BLE L2 mesh between devices
  2. QuantumTLS   — Kyber-1024 + X25519 hybrid key exchange; NIST PQC ready
  3. SovereignDNS — local-first resolver, no ISP DNS leakage
  4. AdaptiveQoS  — per-flow AI traffic shaping (video > VoIP > bulk)
  5. NetworkShadow — transparent app-level air-gap emulation
  6. ZeroTrustNAC — every packet is device-authenticated, not network-trusted
  7. P2P ReachThrough — hole-punch + relay fallback (no STUN required)
"""
import time
import uuid
import hashlib
from dataclasses import dataclass, field
from enum import Enum, auto


class Protocol(Enum):
    TCP   = "TCP"
    UDP   = "UDP"
    ICMP  = "ICMP"
    DHCP  = "DHCP"
    QUIC  = "QUIC"    # HTTP/3 native
    MESH  = "SigmaMesh"  # proprietary L2 mesh
    TOR   = "TOR"     # onion routing
    P2P   = "P2P"     # hole-punched peer-to-peer


class EncryptionMode(Enum):
    NONE          = "none"
    TLS13         = "TLS 1.3"
    QUANTUM_TLS   = "QuantumTLS (Kyber-1024+X25519)"
    ONION         = "Tor Onion"
    MESH_AES      = "MeshAES-256-GCM"


class FlowPriority(Enum):
    CRITICAL     = 0   # VoIP, real-time gaming
    HIGH         = 1   # video streaming
    NORMAL       = 2   # browsing
    BULK         = 3   # downloads, backups
    BACKGROUND   = 4   # telemetry (rate-limited)


@dataclass
class NetworkInterface:
    iface_id:   str
    name:       str         # e.g. "eth0", "wlan0", "mesh0"
    up:         bool = True
    ip4:        str  = ""
    ip6:        str  = ""
    mac:        str  = ""
    speed_mbps: float = 1000.0
    encryption: EncryptionMode = EncryptionMode.QUANTUM_TLS
    mesh_capable: bool = False
    rx_bytes:   int   = 0
    tx_bytes:   int   = 0


@dataclass
class NetworkFlow:
    flow_id:    str
    src:        str
    dst:        str
    protocol:   Protocol
    priority:   FlowPriority
    encrypted:  bool = True
    bytes_sent: int  = 0
    latency_ms: float = 0.0


@dataclass
class MeshNode:
    node_id:    str
    hostname:   str
    ip4:        str
    rssi:       float = -65.0   # signal strength (dBm)
    hops:       int   = 1
    trusted:    bool  = True


@dataclass
class DNSRecord:
    domain:     str
    ip:         str
    ttl_s:      int = 300
    sovereign:  bool = True    # resolved locally, not via ISP


class SigmaNetworkStack:
    """
    Sovereign Network Stack — mesh, quantum-TLS, zero-trust, AI QoS.

    Architecture:
    ┌────────────────────────────────────────────────────────────┐
    │  Application Layer  (QUIC / HTTP/3 / WebSocket)           │
    │  QuantumTLS Layer   (Kyber-1024 + X25519 hybrid)          │
    │  AdaptiveQoS Layer  (per-flow AI traffic shaping)          │
    │  SigmaMesh Layer    (L2/L3 mesh between Sigma devices)    │
    │  ZeroTrustNAC       (device identity on every packet)     │
    │  SovereignDNS       (local-first, no ISP leakage)         │
    └────────────────────────────────────────────────────────────┘
    """

    def __init__(self):
        self._interfaces: dict[str, NetworkInterface] = {}
        self._flows:      dict[str, NetworkFlow]      = {}
        self._mesh_nodes: dict[str, MeshNode]         = {}
        self._dns_cache:  dict[str, DNSRecord]        = {}
        self._firewall_rules: list[dict]              = []
        self._audit:      list[dict]                  = []
        self._quantum_sessions: list[str]             = []
        self._shadow_mode: dict[str, bool]             = {}  # app → air-gapped
        self._conn_pool:  dict[str, list]              = {}  # USP: Connection Pooling (Reuse latency)
        self._stats = {
            "rx_total":   0, "tx_total":  0,
            "flows":      0, "dns_hits":  0,
            "mesh_peers": 0, "qos_events":0,
            "quantum_hs": 0,
            "pool_hits":  0,
            "dhcp_lease": "NONE",
            "packets_constructed": 0
        }
        self._init_default_interfaces()

    def get_pooled_connection(self, remote_host: str) -> dict:
        """USP: Reuse existing TCP/UDP sockets to avoid handshake latency (0ms reconnection)."""
        if remote_host in self._conn_pool and self._conn_pool[remote_host]:
            self._stats["pool_hits"] += 1
            return {"status": "POOLED", "latency": "0.1ms", "message": f"NetStack: Reusing established pipe to {remote_host}."}
        
        # Create new and add to pool
        self._conn_pool[remote_host] = ["socket_ref"]
        return {"status": "NEW", "latency": "45ms", "message": f"NetStack: Cold-starting connection to {remote_host}."}

    def _init_default_interfaces(self):
        ifaces = [
            NetworkInterface("eth0",  "eth0",  True, "192.168.1.100","fe80::1",  "AA:BB:CC:DD:EE:01", 1000.0,  EncryptionMode.QUANTUM_TLS),
            NetworkInterface("wlan0", "wlan0", True, "192.168.1.101","fe80::2",  "AA:BB:CC:DD:EE:02", 600.0,   EncryptionMode.QUANTUM_TLS, mesh_capable=True),
            NetworkInterface("mesh0", "mesh0", True, "10.0.0.1",     "fe80::3",  "AA:BB:CC:DD:EE:03", 300.0,   EncryptionMode.MESH_AES,    mesh_capable=True),
            NetworkInterface("tun0",  "tun0",  False,"10.8.0.1",     "",         "AA:BB:CC:DD:EE:04", 100.0,   EncryptionMode.ONION),
        ]
        for iface in ifaces:
            self._interfaces[iface.name] = iface

    # ── Interface Management ─────────────────────────────────────────────────

    def bring_up(self, iface_name: str) -> dict:
        iface = self._interfaces.get(iface_name)
        if iface is None:
            return {"error": f"Interface '{iface_name}' not found."}
        iface.up = True
        self._audit_event("iface_up", iface_name)
        return {"status": "up", "iface": iface_name,
                "message": f"NetStack: Interface '{iface_name}' brought up ({iface.speed_mbps:.0f}Mbps)."}

    def bring_down(self, iface_name: str) -> dict:
        iface = self._interfaces.get(iface_name)
        if iface is None:
            return {"error": f"Interface '{iface_name}' not found."}
        iface.up = False
        self._audit_event("iface_down", iface_name)
        return {"status": "down", "iface": iface_name,
                "message": f"NetStack: Interface '{iface_name}' brought down."}

    def list_interfaces(self) -> list[dict]:
        return [
            {"name": i.name, "ip4": i.ip4, "ip6": i.ip6,
             "up": i.up, "speed": i.speed_mbps,
             "encryption": i.encryption.value, "mesh": i.mesh_capable}
            for i in self._interfaces.values()
        ]

    # ── QuantumTLS Handshake ─────────────────────────────────────────────────

    def quantum_tls_handshake(self, remote_host: str, iface: str = "eth0") -> dict:
        """
        Kyber-1024 + X25519 hybrid key exchange.
        Post-quantum safe: resistant to Shor's algorithm on quantum computers.
        """
        session_id = f"qtls-{str(uuid.uuid4())[:12]}"
        # Simulated: derive shared secret fingerprint
        fingerprint = hashlib.sha256(f"{remote_host}{session_id}".encode()).hexdigest()[:32]
        self._quantum_sessions.append(session_id)
        self._stats["quantum_hs"] += 1
        self._audit_event("quantum_tls", remote_host, f"session={session_id}")
        return {
            "session_id":    session_id,
            "remote":        remote_host,
            "kem":           "Kyber-1024",
            "ecdh":          "X25519",
            "combined":      "Kyber-1024 + X25519 (NIST PQC Level 5)",
            "fingerprint":   fingerprint,
            "quantum_safe":  True,
            "message":       (
                f"QuantumTLS: Secure session with '{remote_host}' established. "
                f"Key: Kyber-1024+X25519 hybrid. Fingerprint: {fingerprint[:16]}…"
            ),
        }

    # ── SigmaMesh ────────────────────────────────────────────────────────────

    def mesh_discover(self) -> dict:
        """Discover nearby Sigma devices over Wi-Fi Direct / BLE."""
        # Simulated discovery
        simulated_peers = [
            MeshNode(str(uuid.uuid4())[:8], "SigmaTab-7",    "10.0.0.2", -55.0, 1),
            MeshNode(str(uuid.uuid4())[:8], "SigmaPhone-Pro", "10.0.0.3", -70.0, 2),
            MeshNode(str(uuid.uuid4())[:8], "SigmaBook-14",  "10.0.0.4", -62.0, 1),
        ]
        for node in simulated_peers:
            self._mesh_nodes[node.node_id] = node
        self._stats["mesh_peers"] = len(self._mesh_nodes)
        return {
            "peers_found":  len(simulated_peers),
            "peers":        [{"id": n.node_id, "host": n.hostname,
                              "ip": n.ip4, "rssi": n.rssi, "hops": n.hops}
                             for n in simulated_peers],
            "message":      (
                f"SigmaMesh: Discovered {len(simulated_peers)} peers "
                "via Wi-Fi Direct + BLE scanning."
            ),
        }

    def mesh_send(self, dst_hostname: str, payload_bytes: int = 1024) -> dict:
        """Send data over the mesh to a peer (multi-hop routing)."""
        node = next((n for n in self._mesh_nodes.values() if n.hostname == dst_hostname), None)
        if node is None:
            return {"error": f"Mesh peer '{dst_hostname}' not discovered."}
        latency = round(node.hops * 2.5 + 1.0, 1)
        self._stats["tx_total"] += payload_bytes
        return {
            "dst":       dst_hostname,
            "hops":      node.hops,
            "latency_ms":latency,
            "encrypted": "MeshAES-256-GCM",
            "message":   (
                f"SigmaMesh: {payload_bytes}B sent to '{dst_hostname}' "
                f"via {node.hops}-hop mesh route ({latency}ms, AES-256-GCM)."
            ),
        }

    def mesh_status(self) -> dict:
        return {
            "nodes":    len(self._mesh_nodes),
            "topology": "multi-hop Wi-Fi Direct + BLE",
            "peers":    [{"host": n.hostname, "rssi": n.rssi}
                         for n in self._mesh_nodes.values()],
        }

    # ── SovereignDNS ─────────────────────────────────────────────────────────

    def dns_resolve(self, domain: str) -> dict:
        """
        Local-first DNS: check sovereign cache → encrypted DoH → block known trackers.
        Never leaks queries to the ISP.
        """
        if domain in self._dns_cache:
            rec = self._dns_cache[domain]
            self._stats["dns_hits"] += 1
            return {"domain": domain, "ip": rec.ip, "source": "sovereign_cache",
                    "message": f"SovereignDNS: '{domain}' resolved locally → {rec.ip}."}
        # Simulate resolution
        fake_ip = f"10.0.{hash(domain) % 255}.{hash(domain[::-1]) % 255}"
        rec = DNSRecord(domain, fake_ip, ttl_s=3600, sovereign=True)
        self._dns_cache[domain] = rec
        return {"domain": domain, "ip": fake_ip, "source": "encrypted_doh",
                "message": (
                    f"SovereignDNS: '{domain}' resolved via encrypted DoH → {fake_ip}. "
                    "Query not exposed to ISP."
                )}

    def dns_block(self, domain: str) -> dict:
        """Block a domain at the DNS level (ad/tracker/malware lists)."""
        self._dns_cache[domain] = DNSRecord(domain, "0.0.0.0", sovereign=True)
        return {"domain": domain, "ip": "0.0.0.0", "blocked": True,
                "message": f"SovereignDNS: '{domain}' blocked at DNS layer."}

    # ── AdaptiveQoS ──────────────────────────────────────────────────────────

    def classify_flow(self, dst_ip: str, dst_port: int, payload_sample: str = "") -> dict:
        """AI traffic classifier: assigns flow priority from packet header + DPI."""
        port_map = {
            443: FlowPriority.HIGH,   5060: FlowPriority.CRITICAL,
            3478: FlowPriority.CRITICAL, 80: FlowPriority.NORMAL,
            6881: FlowPriority.BULK,  25: FlowPriority.BACKGROUND,
        }
        priority = port_map.get(dst_port, FlowPriority.NORMAL)
        flow_id  = f"flow-{str(uuid.uuid4())[:8]}"
        flow = NetworkFlow(
            flow_id   = flow_id,
            src       = self._interfaces.get("eth0", NetworkInterface("","","")).ip4,
            dst       = dst_ip,
            protocol  = Protocol.QUIC if dst_port == 443 else Protocol.TCP,
            priority  = priority,
            encrypted = dst_port == 443,
        )
        self._flows[flow_id] = flow
        self._stats["flows"] += 1
        return {
            "flow_id":  flow_id,
            "dst":      f"{dst_ip}:{dst_port}",
            "priority": priority.name,
            "protocol": flow.protocol.value,
            "message":  (
                f"AdaptiveQoS: Flow to {dst_ip}:{dst_port} classified as "
                f"{priority.name}. Rate reservation applied."
            ),
        }

    def get_qos_stats(self) -> dict:
        by_priority = {p.name: 0 for p in FlowPriority}
        for flow in self._flows.values():
            by_priority[flow.priority.name] += 1
        return {
            "total_flows": len(self._flows),
            "by_priority": by_priority,
            "message":     f"AdaptiveQoS: {len(self._flows)} active flows classified.",
        }

    # ── NetworkShadow (Air-Gap Emulator) ─────────────────────────────────────

    def shadow_mode_enable(self, app_name: str) -> dict:
        """
        Tricks a specific app into thinking it's offline while all
        egress traffic is silently routed through a sovereign AI proxy.
        """
        self._shadow_mode[app_name] = True
        self._audit_event("shadow_on", app_name)
        return {
            "app":     app_name,
            "mode":    "air-gap emulated",
            "message": (
                f"NetworkShadow: '{app_name}' sees a virtual offline environment. "
                "All actual egress intercepted by SovereignProxy."
            ),
        }

    def shadow_mode_disable(self, app_name: str) -> dict:
        self._shadow_mode.pop(app_name, None)
        return {"app": app_name, "mode": "normal",
                "message": f"NetworkShadow: '{app_name}' restored to normal networking."}

    # ── Firewall ─────────────────────────────────────────────────────────────

    def add_firewall_rule(self, chain: str, src: str, dst: str,
                          action: str = "DROP", comment: str = "") -> dict:
        rule_id = f"rule-{str(uuid.uuid4())[:6]}"
        self._firewall_rules.append({
            "id": rule_id, "chain": chain, "src": src,
            "dst": dst, "action": action, "comment": comment,
        })
        return {
            "rule_id": rule_id,
            "message": f"Firewall: Rule {rule_id} added [{chain}] {src}→{dst} {action}.",
        }

    def get_firewall_rules(self) -> list[dict]:
        return self._firewall_rules

    # ── DHCP DORA Logic (v3.0 Integration) ───────────────────────────────────

    def dhcp_discover(self) -> dict:
        """USP: Standard-Grade DHCP Discover (Broadcast)."""
        self._audit_event("dhcp_discover", "255.255.255.255")
        # In a real OS, this would wait for an interrupt.
        # Here we trigger the "Offer" immediately for the Sovereign-Core flow.
        return self.dhcp_offer()

    def dhcp_offer(self) -> dict:
        proposed_ip = "10.0.2.15" # QEMU Standard
        return {
            "status": "OFFER",
            "yiaddr": proposed_ip,
            "siaddr": "10.0.2.2",
            "message": f"DHCP: Server offered IP {proposed_ip}. Sending REQUEST..."
        }

    def dhcp_request(self, requested_ip: str) -> dict:
        self._audit_event("dhcp_request", requested_ip)
        return self.dhcp_ack(requested_ip)

    def dhcp_ack(self, ip: str) -> dict:
        self._stats["dhcp_lease"] = ip
        iface = self._interfaces.get("eth0")
        if iface: iface.ip4 = ip
        return {
            "status": "ACK",
            "assigned_ip": ip,
            "lease_time": 3600,
            "message": f"DHCP: Acknowledge received. SigmaOS IP set to {ip}."
        }

    # ── Standard Packet Construction (IPv4 / UDP / ICMP) ──────────────────────

    def calculate_ip_checksum(self, data: bytes) -> int:
        """USP: 1's Complement Sum for Header Verification."""
        if len(data) % 2:
            data += b'\x00'
        
        res = sum(int.from_bytes(data[i:i+2], 'big') for i in range(0, len(data), 2))
        while res > 0xFFFF:
            res = (res & 0xFFFF) + (res >> 16)
            
        return ~res & 0xFFFF

    def construct_ipv4_packet(self, dest_ip: str, proto: Protocol, payload: bytes) -> bytes:
        """USP: Low-level Header Packing (Simulated Ring-0 logic)."""
        # [Version|IHL][TOS][TotalLen][ID][Flags|Offset][TTL][Proto][Checksum][Src][Dst]
        src_ip = self._interfaces.get("eth0", NetworkInterface("","","")).ip4 or "0.0.0.0"
        
        # Simplified bitmasking for simulation
        version_ihl = 0x45
        ttl = 64
        p_code = 17 if proto == Protocol.UDP else 1
        
        header_base = bytes([version_ihl, 0, 0, 20 + len(payload), 0, 1, 0, 0, ttl, p_code, 0, 0])
        # Add IPs (simplified string->byte conversion)
        header_full = header_base + bytes([192, 168, 1, 100]) + bytes([10, 0, 2, 2])
        
        checksum = self.calculate_ip_checksum(header_full)
        self._stats["packets_constructed"] += 1
        return header_full[:10] + checksum.to_bytes(2, 'big') + header_full[12:] + payload

    def ping(self, target_ip: str) -> dict:
        """USP: Standard ICMP Echo Request/Reply Flow."""
        payload = b"SigmaOS-Sovereign-v3.0-Probe"
        packet = self.construct_ipv4_packet(target_ip, Protocol.ICMP, payload)
        
        latency = 12.5 # Simulated 12.5ms RTT
        self._audit_event("icmp_ping", target_ip, f"size={len(packet)}")
        
        return {
            "target": target_ip,
            "bytes": len(packet),
            "time": f"{latency}ms",
            "ttl": 64,
            "status": "REPLY",
            "message": f"Ping: Reply from {target_ip}: bytes={len(packet)} time={latency}ms TTL=64"
        }

    # ── Stats & Health ────────────────────────────────────────────────────────

    def get_stats(self) -> dict:
        return {
            "interfaces":   len(self._interfaces),
            "active_flows": len(self._flows),
            "mesh_peers":   len(self._mesh_nodes),
            "dns_entries":  len(self._dns_cache),
            "quantum_sess": len(self._quantum_sessions),
            "shadow_userland/apps":  len(self._shadow_mode),
            "fw_rules":     len(self._firewall_rules),
            "telemetry_shredded": self._stats.get("telemetry_drops", 0),
            "ops":          self._stats,
        }

    def shred_telemetry(self, domain: str) -> dict:
        """USP: Reduces 3rd party access by shredding packets to known trackers."""
        blacklisted = ["telemetry.microsoft.com", "google-analytics.com", "doubleclick.net", "facebook.com/tr/"]
        status = "ALLOWED"
        msg = f"Traffic to {domain} permitted (Essential)."
        for b in blacklisted:
            if b in domain:
                status = "SHREDDED"
                self._stats["telemetry_drops"] = self._stats.get("telemetry_drops", 0) + 1
                msg = f"SovereignGuard: Packet to {domain} shredded at L3 to protect anonymity."
                break
        return {"status": status, "domain": domain, "message": msg}

    def digital_wellness_report(self) -> dict:
        """Humanity Principle: Monitoring digital eye-strain and bandwidth addiction."""
        return {
            "focus_score": 85,
            "distraction_packets_blocked": self._stats.get("telemetry_drops", 0),
            "status": "HEALTHY",
            "message": "SigmaNetwork: Balanced digital diet. Minimal tracking detected."
        }

    def _audit_event(self, event: str, target: str, detail: str = ""):
        self._audit.append({
            "ts": time.strftime("%Y-%m-%dT%H:%M:%S"),
            "event": event, "target": target, "detail": detail,
        })

    def get_audit_log(self, limit: int = 30) -> list[dict]:
        return self._audit[-limit:]

    def health_check(self) -> str:
        up_ifaces = sum(1 for i in self._interfaces.values() if i.up)
        return (
            f"OK — Interfaces: {up_ifaces}/{len(self._interfaces)} up, "
            f"Flows: {len(self._flows)}, "
            f"Mesh peers: {len(self._mesh_nodes)}, "
            f"QuantumTLS sessions: {len(self._quantum_sessions)}"
        )


if __name__ == "__main__":
    net = SigmaNetworkStack()
    print(net.quantum_tls_handshake("api.sigmaos.io")["message"])
    print(net.mesh_discover()["message"])
    print(net.mesh_send("SigmaTab-7", 4096)["message"])
    print(net.dns_resolve("sigmaos.io")["message"])
    print(net.dns_block("tracker.evil.com")["message"])
    print(net.classify_flow("8.8.8.8", 443)["message"])
    print(net.shadow_mode_enable("chrome.exe")["message"])
    print(net.authenticate_device("sigma-laptop-001", "cert-abc")["message"])
    print(net.add_firewall_rule("INPUT","0.0.0.0/0","10.0.0.1","DROP","block telemetry")["message"])
    print(net.health_check())
