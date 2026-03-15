# Generated method: ChatActions.switch_alias
from ..protocol import SecurePacket

class ChatActions:
    @staticmethod
    def switch_alias(engine, new_alias: str):
        engine.identity.alias = new_alias
        engine._purge_volatile_memory()
        return f"Identity shifted to '{new_alias}'. Session keys rotated."