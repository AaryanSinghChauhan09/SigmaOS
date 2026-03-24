# Generated method: SigmaCustomizer.__init__
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaJSON as _json_lib
    class json:
        loads = staticmethod(_json_lib.loads)
        dumps = staticmethod(_json_lib.dumps)
        load  = staticmethod(lambda f: _json_lib.loads(f.read()))
        dump  = staticmethod(lambda d, f, **kw: f.write(_json_lib.dumps(d)))
except Exception:
    import json
import os as _os, sys as _sys
_sigma_root = _os.path.abspath(_os.path.join(_os.path.dirname(__file__) or '.', '..', '..', '..', '..'))
if _sigma_root not in _sys.path: _sys.path.insert(0, _sigma_root)
try:
    from sigma_core.sigma_libc import SigmaEntropy as _ent
    class random:
        @staticmethod
        def random(): return _ent.randint(0, 10**9) / 10**9
        @staticmethod
        def randint(a, b): return _ent.randint(a, b)
        @staticmethod
        def uniform(a, b): return a + (b - a) * (random.random())
        @staticmethod
        def choice(seq): return seq[_ent.randint(0, len(seq)-1)] if seq else None
        @staticmethod
        def shuffle(lst):
            for i in range(len(lst)-1, 0, -1):
                j = _ent.randint(0, i); lst[i], lst[j] = lst[j], lst[i]
        @staticmethod
        def sample(pop, k): return [pop[_ent.randint(0,len(pop)-1)] for _ in range(k)]
except Exception:
    import random
import os

class SigmaCustomizer:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.active_theme = 'Sovereign_Dark'
        self._styles = {'blur_radius': 20, 'accent_color': '#00FFC2', 'background_color': '#0A0A0B', 'transparency': 0.85, 'font_scaling': 1.0, 'font_weight': 'Regular', 'animation_speed': 'Fluid', 'atom_shader_active': False, 'morph_transition_ms': 300, 'sidebar_position': 'Left', 'dashboard_spacing': 'Comfortable', 'icon_pack': 'Sovereign_3D', 'soundscape': 'Calm_Ethereal', 'animation_curve': 'Ease-InOut-Quartic', 'molecular_css_layer': 'Default_Sovereign', 'material_engine': 'Inactive', 'dynamic_desktop': 'Inactive', 'mica_blur': 'Inactive'}
        self._stats = {'widgets_forged': 12, 'themes_generated': 5, 'icons_mutated': 450}