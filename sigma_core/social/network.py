"""
SigmaOS Sovereign Networking Module
===================================
Handles low-level UDP socket orchestration for the mesh.
"""
import socket
from typing import Optional

class MeshSocket:
    def __init__(self, port: int):
        self.port = port
        self.sock: Optional[socket.socket] = None

    def bind(self):
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.setblocking(False)
        self.sock.bind(("0.0.0.0", self.port))
        return self.sock

    def send(self, ip: str, port: int, data: bytes):
        if self.sock:
            self.sock.sendto(data, (ip, port))

    def close(self):
        if self.sock:
            self.sock.close()
