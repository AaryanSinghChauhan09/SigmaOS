# Generated method: SigmaNetworkStack.construct_ipv4_packet
import time
import uuid
import hashlib
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaNetworkStack:
    def construct_ipv4_packet(self, dest_ip: str, proto: Protocol, payload: bytes) -> bytes:
        """USP: Low-level Header Packing (Simulated Ring-0 logic)."""
        src_ip = self._interfaces.get('eth0', NetworkInterface('', '', '')).ip4 or '0.0.0.0'
        version_ihl = 69
        ttl = 64
        p_code = 17 if proto == Protocol.UDP else 1
        header_base = bytes([version_ihl, 0, 0, 20 + len(payload), 0, 1, 0, 0, ttl, p_code, 0, 0])
        header_full = header_base + bytes([192, 168, 1, 100]) + bytes([10, 0, 2, 2])
        checksum = self.calculate_ip_checksum(header_full)
        self._stats['packets_constructed'] += 1
        return header_full[:10] + checksum.to_bytes(2, 'big') + header_full[12:] + payload