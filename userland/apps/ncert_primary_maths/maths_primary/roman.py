# Generated method: Maths_Primary.roman


class Maths_Primary:
    @staticmethod
    def roman(n):
        d = {1: 'I', 2: 'II', 3: 'III', 4: 'IV', 5: 'V', 6: 'VI', 7: 'VII', 8: 'VIII', 9: 'IX', 10: 'X'}
        return {'Roman': d.get(int(n), 'Out of Range')}