# Generated method: Science_Primary_Classes.neighborhood


class Science_Primary_Classes:
    @staticmethod
    def neighborhood(p):
        d = {'hospital': 'Treat', 'bank': 'Safe', 'post': 'Letters'}
        return {'Duty': d.get(p.lower(), 'Help')}