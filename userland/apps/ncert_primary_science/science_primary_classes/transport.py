# Generated method: Science_Primary_Classes.transport


class Science_Primary_Classes:
    @staticmethod
    def transport(v):
        d = {'car': 'Land', 'boat': 'Water', 'plane': 'Air'}
        return {'Path': d.get(v.lower(), 'Land')}