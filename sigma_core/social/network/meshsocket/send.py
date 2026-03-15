# Generated method: MeshSocket.send
import socket
from typing import Optional

class MeshSocket:
    def send(self, ip: str, port: int, data: bytes):
        if self.sock:
            self.sock.sendto(data, (ip, port))