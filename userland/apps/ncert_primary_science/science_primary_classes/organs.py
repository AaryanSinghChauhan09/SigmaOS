# Generated method: Science_Primary_Classes.organs


class Science_Primary_Classes:
    @staticmethod
    def organs(p):
        d = {'heart': 'Pumps Blood', 'lungs': 'Breathe', 'stomach': 'Digests'}
        return {'Function': d.get(p.lower(), 'Supports Life')}