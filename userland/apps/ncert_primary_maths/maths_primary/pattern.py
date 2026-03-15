# Generated method: Maths_Primary.pattern


class Maths_Primary:
    @staticmethod
    def pattern(s):
        n = [int(x.strip()) for x in s.split(',')]
        return {'Next': n[-1] + (n[1] - n[0])}