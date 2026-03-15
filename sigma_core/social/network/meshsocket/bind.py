# Generated method: MeshSocket.bind
import socket
from typing import Optional

class MeshSocket:
    def bind(self):
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.setblocking(False)
        self.sock.bind(('0.0.0.0', self.port))
        return self.sock