# Generated method: Science_Primary_Classes.senses


class Science_Primary_Classes:
    @staticmethod
    def senses(o):
        d = {'eyes': 'Vision', 'ears': 'Hearing', 'nose': 'Smell', 'tongue': 'Taste', 'skin': 'Touch'}
        return {'Role': d.get(o.lower(), 'Sensing')}