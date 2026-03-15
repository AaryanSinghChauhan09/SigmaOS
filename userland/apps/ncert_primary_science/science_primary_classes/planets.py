# Generated method: Science_Primary_Classes.planets


class Science_Primary_Classes:
    @staticmethod
    def planets(p):
        d = {1: 'Mercury', 2: 'Venus', 3: 'Earth', 4: 'Mars', 5: 'Jupiter', 6: 'Saturn', 7: 'Uranus', 8: 'Neptune'}
        return {'Planet': d.get(int(p), 'Pluto (Dwarf)')}