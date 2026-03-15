# Generated method: Maths_Primary.sort


class Maths_Primary:
    @staticmethod
    def sort(s):
        n = [int(x.strip()) for x in s.split(',')]
        return {'Ascending': sorted(n), 'Descending': sorted(n, reverse=True)}