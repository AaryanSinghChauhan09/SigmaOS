# Generated method: Science_Primary_Classes.matter


class Science_Primary_Classes:
    @staticmethod
    def matter(e):
        e = e.lower()
        if 'ice' in e:
            return {'State': 'Solid'}
        if 'water' in e:
            return {'State': 'Liquid'}
        return {'State': 'Gas'}