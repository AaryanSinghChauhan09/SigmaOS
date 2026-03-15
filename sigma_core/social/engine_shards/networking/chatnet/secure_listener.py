# Generated method: ChatNet.secure_listener
import socket
import threading
import json
from userland.system_api.sigma_std import SigmaCrypto

class ChatNet:
    @staticmethod
    def secure_listener(engine):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            try:
                s.bind(('0.0.0.0', engine.port))
                s.listen(10)
                while engine._running:
                    conn, addr = s.accept()
                    threading.Thread(target=engine._handle_secure_conn, args=(conn, addr), daemon=True).start()
            except Exception as e:
                engine.log_event('LISTENER_FAIL', str(e), 'ERR')