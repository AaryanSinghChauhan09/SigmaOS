# Generated method: MeshSocket.close
import socket
from typing import Optional

class MeshSocket:
    def close(self):
        if self.sock:
            self.sock.close()