# Generated method: Maths_Primary.money


class Maths_Primary:
    @staticmethod
    def money(rs, ps, c):
        total = (int(rs) * 100 + int(ps)) * int(c)
        return {'Total Rs': total / 100, 'Notes/Coins': f'Total {c} items cost Rs {total / 100}'}