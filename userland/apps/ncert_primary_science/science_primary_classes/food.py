# Generated method: Science_Primary_Classes.food


class Science_Primary_Classes:
    @staticmethod
    def food(i):
        i = i.lower()
        if i in ['rice', 'wheat']:
            return {'Group': 'Energy'}
        if i in ['milk', 'egg']:
            return {'Group': 'Body'}
        return {'Group': 'Protective'}