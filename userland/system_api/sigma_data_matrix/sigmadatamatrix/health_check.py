# Generated method: SigmaDataMatrix.health_check


class SigmaDataMatrix:
    def health_check(self) -> str:
        s = 'OK' if self.active_dataframe else 'Empty'
        return f'OK — Sigma Data Matrix Active. Kernel State: {s}.'