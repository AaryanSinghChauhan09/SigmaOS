# Generated method: Maths_Primary.abacus


class Maths_Primary:
    @staticmethod
    def abacus(n):
        n = int(n)
        return {'Tens': n // 10, 'Ones': n % 10}