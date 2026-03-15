

class DefensiveGuard:
    @staticmethod
    def validate_shard_id(shard_id):
        if not isinstance(shard_id, str):
            return False
        if len(shard_id) > 255:
            return False
        return True