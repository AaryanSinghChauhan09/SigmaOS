# Generated method: MeshSocket.__init__
import socket
from typing import Optional

class MeshSocket:
    def __init__(self, port: int):
        self.port = port
        self.sock: Optional[socket.socket] = None