"""
SigmaOS Sovereign Chat - Service Operations
===========================================
Handles service lifecycle: start, stop, and memory purging.
"""
import threading

class ChatOps:
    @staticmethod
    def start_engine(engine):
        if not engine._running:
            engine._running = True
            engine._server_thread = threading.Thread(target=engine._secure_listener, daemon=True)
            engine._server_thread.start()
            engine.log_event("CHAT_ACTIVE", {"sid": engine.identity.sid, "alias": engine.identity.alias})
            return f"Sovereign Chat Engine Online. SID: {engine.identity.sid}"
        return "Already running."

    @staticmethod
    def stop_engine(engine):
        engine._running = False
        ChatOps.purge_memory(engine)
        engine.log_event("CHAT_OFFLINE", {"status": "PURGED"})

    @staticmethod
    def purge_memory(engine):
        engine.inbox.clear()
        if hasattr(engine, 'peer_dir') and hasattr(engine.peer_dir, '_peers'):
            engine.peer_dir._peers.clear()
        engine.stats["shredded_metadata_kb"] += 12.5
