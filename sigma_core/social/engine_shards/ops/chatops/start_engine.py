# Generated method: ChatOps.start_engine
import threading

class ChatOps:
    @staticmethod
    def start_engine(engine):
        if not engine._running:
            engine._running = True
            engine._server_thread = threading.Thread(target=engine._secure_listener, daemon=True)
            engine._server_thread.start()
            engine.log_event('CHAT_ACTIVE', {'sid': engine.identity.sid, 'alias': engine.identity.alias})
            return f'Sovereign Chat Engine Online. SID: {engine.identity.sid}'
        return 'Already running.'