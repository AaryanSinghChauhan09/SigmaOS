# Generated method: ChatLogic.process_message
import time
from typing import Dict
from userland.system_api.sigma_std import SigmaCrypto

class ChatLogic:
    @staticmethod
    def process_message(engine, packet: Dict):
        sender_sid = packet.get('from')
        peer_info = engine.peer_dir.get_peer(sender_sid)
        if not peer_info:
            return
        enc_payload = bytes.fromhex(str(packet.get('payload', '')))
        decrypted_text = SigmaCrypto.decrypt_payload(enc_payload, peer_info['shared_secret'])
        if not decrypted_text.startswith('DEC_ERR'):
            msg_obj = {'sid': sender_sid, 'text': decrypted_text, 'time': time.time(), 'verified': True}
            engine.inbox.append(msg_obj)
            engine.stats['packets_decrypted'] += 1
            if engine.kernel:
                engine.kernel.bus.publish('social.chat.msg', msg_obj)