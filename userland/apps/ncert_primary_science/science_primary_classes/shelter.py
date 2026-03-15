# Generated method: Science_Primary_Classes.shelter


class Science_Primary_Classes:
    @staticmethod
    def shelter(a):
        d = {'mountain': 'Sloping', 'desert': 'Mud', 'river': 'Boat'}
        return {'Roof': d.get(a.lower(), 'Flat')}