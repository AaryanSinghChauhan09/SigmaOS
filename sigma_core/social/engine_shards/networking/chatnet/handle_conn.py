# Generated method: ChatNet.handle_conn
import socket
import threading
import json
from userland.system_api.sigma_std import SigmaCrypto

class ChatNet:
    @staticmethod
    def handle_conn(engine, conn, addr):
        with conn:
            try:
                raw_data = conn.recv(8192)
                if not raw_data:
                    return
                packet = json.loads(raw_data.decode('utf-8', errors='ignore'))
                p_type = packet.get('type')
                sender_sid = packet.get('from', 'UNKNOWN')
                nonce = packet.get('nonce', '')
                if not SigmaCrypto.verify_pow(sender_sid + p_type, nonce, difficulty=3):
                    engine.log_event('PACKET_REJECTED', 'Invalid PoW')
                    return
                engine.stats['pow_verifications'] += 1
                if p_type == 'HANDSHAKE':
                    engine._process_handshake(packet, addr[0])
                elif p_type == 'SECURE_MSG' or p_type == 'CHANNEL_MSG' or p_type == 'MESSAGE':
                    engine._process_encrypted_message(packet)
            except Exception as e:
                pass