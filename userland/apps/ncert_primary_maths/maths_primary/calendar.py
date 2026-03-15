# Generated method: Maths_Primary.calendar


class Maths_Primary:
    @staticmethod
    def calendar(m, y):
        m, y = (m.lower(), int(y))
        if 'feb' in m:
            return {'Days': 29 if y % 4 == 0 and y % 100 != 0 or y % 400 == 0 else 28}
        if m in ['april', 'june', 'sept', 'nov']:
            return {'Days': 30}
        return {'Days': 31}