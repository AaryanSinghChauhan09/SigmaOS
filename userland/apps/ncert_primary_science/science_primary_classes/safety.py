# Generated method: Science_Primary_Classes.safety


class Science_Primary_Classes:
    @staticmethod
    def safety(s):
        s = s.lower()
        if 'red' in s:
            return {'Action': 'STOP'}
        if 'green' in s:
            return {'Action': 'GO'}
        if 'yellow' in s:
            return {'Action': 'WAIT'}
        return {'Action': 'Be Careful'}