# Generated method: ChatOps.purge_memory
import threading

class ChatOps:
    @staticmethod
    def purge_memory(engine):
        engine.inbox.clear()
        if hasattr(engine, 'peer_dir') and hasattr(engine.peer_dir, '_peers'):
            engine.peer_dir._peers.clear()
        engine.stats['shredded_metadata_kb'] += 12.5