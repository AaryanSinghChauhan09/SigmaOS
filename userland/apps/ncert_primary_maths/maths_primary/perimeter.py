# Generated method: Maths_Primary.perimeter


class Maths_Primary:
    @staticmethod
    def perimeter(sh, s):
        sh = sh.lower()
        s = int(s)
        if 'square' in sh:
            return {'Perimeter': 4 * s}
        if 'triangle' in sh:
            return {'Perimeter': 3 * s}
        return {'Perimeter': 'Unknown Shape'}