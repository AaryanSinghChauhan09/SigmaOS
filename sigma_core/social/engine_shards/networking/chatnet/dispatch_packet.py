# Generated method: ChatNet.dispatch_packet
import socket
import threading
import json
from userland.system_api.sigma_std import SigmaCrypto

class ChatNet:
    @staticmethod
    def dispatch_packet(engine, ip: str, packet: dict):
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                s.settimeout(3)
                s.connect((ip, engine.port))
                s.sendall(json.dumps(packet).encode('utf-8'))
        except:
            pass