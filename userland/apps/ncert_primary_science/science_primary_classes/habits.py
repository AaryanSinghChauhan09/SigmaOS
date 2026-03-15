# Generated method: Science_Primary_Classes.habits


class Science_Primary_Classes:
    @staticmethod
    def habits(a):
        good = ['brushing', 'bathing', 'washing', 'studying']
        return {'Quality': 'GOOD' if any((x in a.lower() for x in good)) else 'NEEDS IMPROVEMENT'}