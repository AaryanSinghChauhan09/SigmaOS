"""
SigmaOS Sovereign Chat - Networking Shard
=========================================
Handles the low-level socket listener, connection handling, and message dispatch.
"""
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
                engine.log_event("LISTENER_FAIL", str(e), "ERR")

    @staticmethod
    def handle_conn(engine, conn, addr):
        with conn:
            try:
                raw_data = conn.recv(8192)
                if not raw_data: return
                
                packet = json.loads(raw_data.decode('utf-8', errors='ignore'))
                p_type = packet.get("type")
                sender_sid = packet.get("from", "UNKNOWN")
                nonce = packet.get("nonce", "")
                
                if not SigmaCrypto.verify_pow(sender_sid + p_type, nonce, difficulty=3):
                    engine.log_event("PACKET_REJECTED", "Invalid PoW")
                    return
                
                engine.stats["pow_verifications"] += 1
                
                if p_type == "HANDSHAKE":
                    engine._process_handshake(packet, addr[0])
                elif p_type == "SECURE_MSG" or p_type == "CHANNEL_MSG" or p_type == "MESSAGE":
                    engine._process_encrypted_message(packet)
            except Exception as e:
                pass

    @staticmethod
    def dispatch_packet(engine, ip: str, packet: dict):
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                s.settimeout(3)
                s.connect((ip, engine.port))
                s.sendall(json.dumps(packet).encode('utf-8')) # Bug fixed: data -> packet
        except: pass
