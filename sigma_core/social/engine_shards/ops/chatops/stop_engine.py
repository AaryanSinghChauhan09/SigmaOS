# Generated method: ChatOps.stop_engine
import threading

class ChatOps:
    @staticmethod
    def stop_engine(engine):
        engine._running = False
        ChatOps.purge_memory(engine)
        engine.log_event('CHAT_OFFLINE', {'status': 'PURGED'})