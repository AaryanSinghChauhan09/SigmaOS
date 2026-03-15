# Generated method: Maths_Primary.shapes


class Maths_Primary:
    @staticmethod
    def shapes(s):
        d = {'square': 4, 'triangle': 3, 'circle': 0}
        return {'Sides': d.get(s.lower(), '?')}