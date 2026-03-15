# Generated method: Maths_Primary.sharing


class Maths_Primary:
    @staticmethod
    def sharing(s, t):
        return {'Fraction': f'{s}/{t}', 'Note': 'Quarter' if s / t == 0.25 else 'Half' if s / t == 0.5 else 'Part'}