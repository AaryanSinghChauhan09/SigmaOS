# Generated method: Science_Primary_Classes.pollution


class Science_Primary_Classes:
    @staticmethod
    def pollution(s):
        s = s.lower()
        if 'smoke' in s:
            return {'Type': 'Air Pollution'}
        if 'garbage' in s:
            return {'Type': 'Land Pollution'}
        if 'noise' in s:
            return {'Type': 'Noise Pollution'}
        return {'Type': 'Environmental Harm'}