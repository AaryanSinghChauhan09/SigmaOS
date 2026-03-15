# Generated method: Science_Primary_Classes.magnet


class Science_Primary_Classes:
    @staticmethod
    def magnet(a, b):
        if a.lower() == b.lower():
            return {'Action': 'REPEL', 'Status': 'Same Poles'}
        return {'Action': 'ATTRACT', 'Status': 'Opposite Poles'}