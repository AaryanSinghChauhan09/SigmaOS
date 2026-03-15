# Generated method: Maths_Primary.words


class Maths_Primary:
    @staticmethod
    def words(n):
        d = {0: 'Zero', 1: 'One', 2: 'Two', 3: 'Three', 4: 'Four', 5: 'Five', 6: 'Six', 7: 'Seven', 8: 'Eight', 9: 'Nine'}
        return {'Word': d.get(int(n), 'Big Number')}