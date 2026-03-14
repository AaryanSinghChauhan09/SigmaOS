"""
SigmaOS NCERT Lab Engine v1.0  ─  Kernel-registrable service
=============================================================
Wraps PhysicsLab / ChemistryLab / BiologyLab / MathLab into a
single SigmaModuleBase so the kernel can register it as "ncert_lab".
"""
import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "userland", "apps"))

from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService  # type: ignore


class NCERTLabEngine(SigmaModuleBase, ISigmaService):
    """Kernel-visible NCERT experiment service."""

    def __init__(self, kernel=None):
        if kernel:  # when called directly (non-kernel), skip super().__init__
            super().__init__(kernel)
        self._running = False
        # Lazy-import heavy engine only when needed
        self._phy = self._chem = self._bio = self._math = None

    def _load_engines(self):
        try:
            from ncert_virtual_lab import PhysicsLab, ChemistryLab, BiologyLab, MathLab
            self._phy, self._chem, self._bio, self._math = PhysicsLab, ChemistryLab, BiologyLab, MathLab
        except Exception as e:
            print(f"[NCERT] Engine import error: {e}")

    def start_service(self):
        self._running = True
        self._load_engines()
        return "NCERT Lab Engine: ONLINE"

    def stop_service(self):
        self._running = False

    def health_check(self) -> str:
        return "OK – NCERT Lab: 4 subjects | Classes 1–12"

    # ── Public API (callable from kernel or GUI) ───────────────────
    def run_experiment(self, subject: str, experiment: str, **kwargs) -> dict:
        if not self._phy:
            self._load_engines()
        subj = subject.lower()
        try:
            if subj == "physics":
                return self._phy_dispatch(experiment, **kwargs)
            elif subj == "chemistry":
                return self._chem_dispatch(experiment, **kwargs)
            elif subj == "biology":
                return self._bio_dispatch(experiment, **kwargs)
            elif subj == "mathematics":
                return self._math_dispatch(experiment, **kwargs)
            else:
                return {"error": f"Unknown subject: {subject}"}
        except Exception as e:
            return {"error": str(e)}

    def _phy_dispatch(self, exp, **kw):
        m = self._phy
        dispatchers = {
            "ohms_law":           lambda: m.ohms_law(**kw),
            "projectile":         lambda: m.projectile_motion(**kw),
            "pendulum":           lambda: m.simple_pendulum(**kw),
            "lens":               lambda: m.lens_formula(**kw),
            "snell":              lambda: m.snells_law(**kw),
            "newton2":            lambda: m.newtons_second_law(**kw),
            "boyle":              lambda: m.boyles_law(**kw),
            "coulomb":            lambda: m.coulombs_law(**kw),
            "radioactive":        lambda: m.radioactive_decay(**kw),
            "photoelectric":      lambda: m.photoelectric_effect(**kw),
        }
        return dispatchers.get(exp, lambda: {"error": "Unknown experiment"})()

    def _chem_dispatch(self, exp, **kw):
        m = self._chem
        dispatchers = {
            "molar_mass":   lambda: m.molar_mass(**kw),
            "ph":           lambda: m.ph_calculator(**kw),
            "ideal_gas":    lambda: m.ideal_gas_law(**kw),
            "molarity":     lambda: m.molarity(**kw),
            "enthalpy":     lambda: m.enthalpy_change(**kw),
            "rate":         lambda: m.rate_of_reaction(**kw),
        }
        return dispatchers.get(exp, lambda: {"error": "Unknown experiment"})()

    def _bio_dispatch(self, exp, **kw):
        m = self._bio
        dispatchers = {
            "mendel":       lambda: m.mendel_cross(**kw),
            "photosyn":     lambda: m.photosynthesis_rate(**kw),
            "osmosis":      lambda: m.osmosis(**kw),
            "bmi":          lambda: m.bmi(**kw),
            "microscope":   lambda: m.microscope_magnification(**kw),
            "heart":        lambda: m.heart_rate_zones(**kw),
        }
        return dispatchers.get(exp, lambda: {"error": "Unknown experiment"})()

    def _math_dispatch(self, exp, **kw):
        m = self._math
        dispatchers = {
            "quadratic":    lambda: m.quadratic(**kw),
            "stats":        lambda: m.statistics(**kw),
            "trig":         lambda: m.trig_values(**kw),
            "ap":           lambda: m.ap_terms(**kw),
            "gp":           lambda: m.gp_terms(**kw),
            "permcomb":     lambda: {"P": m.permutation(**kw), "C": m.combination(**kw)},
            "geometry":     lambda: m.geometry(**kw),
            "binomial":     lambda: m.binomial(**kw),
            "complex":      lambda: m.complex_ops(**kw),
        }
        return dispatchers.get(exp, lambda: {"error": "Unknown experiment"})()

    def list_experiments(self, subject: str) -> list:
        try:
            self._load_engines()
            s = subject.lower()
            if s == "physics":    return list(self._phy.EXPERIMENTS.keys())
            if s == "chemistry":  return list(self._chem.EXPERIMENTS.keys())
            if s == "biology":    return list(self._bio.EXPERIMENTS.keys())
            if s == "mathematics":return list(self._math.EXPERIMENTS.keys())
        except Exception:
            pass
        return []
