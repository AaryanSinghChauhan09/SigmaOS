# Generated method: SigmaFamiliarityEngine.get_translated_term


class SigmaFamiliarityEngine:
    def get_translated_term(self, sigma_term: str) -> str:
        if self.active_paradigm == 'Sigma_Sovereign':
            return sigma_term
        return self.paradigms.get(self.active_paradigm, {}).get('terminology', {}).get(sigma_term, sigma_term)