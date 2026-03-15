# Generated method: Maths_Primary.add


class Maths_Primary:
    @staticmethod
    def add(a, b):
        return {'Sum': int(a) + int(b), 'Carry': int(a) % 10 + int(b) % 10 >= 10}