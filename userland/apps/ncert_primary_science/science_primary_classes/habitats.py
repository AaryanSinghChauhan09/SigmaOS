# Generated method: Science_Primary_Classes.habitats


class Science_Primary_Classes:
    @staticmethod
    def habitats(a):
        d = {'fish': 'Water', 'monkey': 'Tree', 'lion': 'Land', 'camel': 'Desert'}
        return {'Home': d.get(a.lower(), 'Forest')}