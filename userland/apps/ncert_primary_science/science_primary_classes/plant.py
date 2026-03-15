# Generated method: Science_Primary_Classes.plant


class Science_Primary_Classes:
    @staticmethod
    def plant(p):
        d = {'root': 'Water', 'leaf': 'Food', 'stem': 'Support'}
        return {'Duty': d.get(p.lower(), 'Growth')}