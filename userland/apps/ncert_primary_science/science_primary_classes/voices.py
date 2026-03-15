# Generated method: Science_Primary_Classes.voices


class Science_Primary_Classes:
    @staticmethod
    def voices(a):
        d = {'dog': 'Bark', 'cat': 'Meow', 'lion': 'Roar', 'cow': 'Moo'}
        return {'Sound': d.get(a.lower(), 'Noise')}