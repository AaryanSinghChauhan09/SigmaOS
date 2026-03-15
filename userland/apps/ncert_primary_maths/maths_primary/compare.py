# Generated method: Maths_Primary.compare


class Maths_Primary:
    @staticmethod
    def compare(x, y):
        x, y = (int(x), int(y))
        if x > y:
            return {'Sign': '>', 'Result': f'{x} is GREATER than {y}'}
        if x < y:
            return {'Sign': '<', 'Result': f'{x} is LESS than {y}'}
        return {'Sign': '=', 'Result': 'EQUAL'}