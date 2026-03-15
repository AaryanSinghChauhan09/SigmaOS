# Generated method: Maths_Primary.skip


class Maths_Primary:
    @staticmethod
    def skip(s, step, c):
        res = [int(s) + i * int(step) for i in range(int(c))]
        return {'Sequence': ', '.join(map(str, res))}