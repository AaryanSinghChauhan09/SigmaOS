

class DefensiveGuard:
    @staticmethod
    def sanitize_path(path):
        if '..' in path or '~' in path:
            return 'ACCESS_DENIED'
        return path