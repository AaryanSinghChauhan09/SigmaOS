# Generated method: Maths_Primary.divide


class Maths_Primary:
    @staticmethod
    def divide(t, f):
        t, f = (int(t), int(f))
        return {'Each child gets': t // f, 'Leftover': t % f}