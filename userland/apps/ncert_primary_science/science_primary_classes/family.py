# Generated method: Science_Primary_Classes.family


class Science_Primary_Classes:
    @staticmethod
    def family(r):
        d = {'father': 'Parent', 'mother': 'Parent', 'brother': 'Sibling', 'sister': 'Sibling'}
        return {'Relationship': d.get(r.lower(), 'Relative')}