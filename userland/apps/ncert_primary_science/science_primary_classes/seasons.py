# Generated method: Science_Primary_Classes.seasons


class Science_Primary_Classes:
    @staticmethod
    def seasons(s):
        d = {'summer': 'Cotton', 'winter': 'Woolen', 'monsoon': 'Rubber'}
        return {'Cloth': d.get(s.lower(), 'Cloth')}