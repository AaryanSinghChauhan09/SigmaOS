

class DefensiveGuard:
    @staticmethod
    def secure_cast_int(value, default=0):
        try:
            return int(value)
        except (ValueError, TypeError):
            return default