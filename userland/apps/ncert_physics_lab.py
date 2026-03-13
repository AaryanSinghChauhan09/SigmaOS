"""
SigmaOS NCERT Physics Lab v8.0 — The Interactive series
Classes 6–12 | Every Core NCERT Experiment & Calculator
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Physics_Classes_6_10:
    TITLE = "Secondary Physics: Interactive Simulations"
    EXP_DATA = {
        "Magnetism (Interaction)": ("magnet", [("Pole 1 (N/S)", "N"), ("Pole 2 (N/S)", "S")]),
        "Light & Shadows": ("shadow", [("Object Distance (cm)", "20"), ("Lamp Height (cm)", "50")]),
        "Electric Circuit (V=IR)": ("ohms_law", [("Voltage (V)", "10"), ("Resistance (Ω)", "5")]),
        "Mirror/Lens Formula": ("optics", [("Type (Mirror/Lens)", "Mirror"), ("Object dist u (cm)", "-20"), ("Focal length f (cm)", "10")]),
        "Gravity & Weight": ("gravity_weight", [("Mass (kg)", "60"), ("Planet (Earth/Moon/Mars/Jupiter)", "Earth")]),
        "Archimedes Principle": ("buoyancy", [("Object Vol (m³)", "0.001"), ("Fluid Density (kg/m³)", "1000")]),
        "Work & Energy": ("energy", [("Mass (kg)", "2"), ("Velocity (m/s)", "10"), ("Height (m)", "5")]),
        "Sound (Tuning Fork)": ("sound_freq", [("Frequency (Hz)", "512"), ("Medium (Air/Water/Iron)", "Air")]),
    }

    @staticmethod
    def magnet(p1, p2):
        p1, p2 = p1.upper(), p2.upper()
        if p1 == p2: return {"Result": "REPEL", "Force": "Magnetic repulsion between like poles"}
        return {"Result": "ATTRACT", "Force": "Magnetic attraction between unlike poles"}

    @staticmethod
    def shadow(d, h):
        d, h = float(d), float(h)
        # s = (h_obj * d_screen) / d_obj
        # simplified ratio
        ratio = h / d
        return {"Shadow Scale Factor": _r(ratio, 2), "Note": "Move object closer to light to increase shadow size"}

    @staticmethod
    def ohms_law(v, r):
        v, r = float(v), float(r)
        return {"Current I (A)": _r(v/r), "Power P (W)": _r(v**2/r)}

    @staticmethod
    def optics(t, u, f):
        u, f = float(u), float(f)
        if "mirror" in t.lower():
            v = 1 / (1/f - 1/u)
            m = -v/u
        else:
            v = 1 / (1/f + 1/u)
            m = v/u
        return {"Image dist v (cm)": _r(v, 2), "Magnification m": _r(m, 2), "Nature": "Real" if v < 0 else "Virtual"}

    @staticmethod
    def gravity_weight(m, p):
        gs = {"earth": 9.8, "moon": 1.62, "mars": 3.71, "jupiter": 24.79}
        g = gs.get(p.lower(), 9.8)
        return {"Weight (N)": _r(float(m)*g, 1), "g (m/s²)": g}

    @staticmethod
    def buoyancy(v, d):
        v, d = float(v), float(d)
        f = v * d * 9.8
        return {"Upthrust (N)": _r(f, 2)}

    @staticmethod
    def energy(m, v, h):
        m, v, h = float(m), float(v), float(h)
        ke = 0.5 * m * v**2
        pe = m * 9.8 * h
        return {"Kinetic (J)": _r(ke, 2), "Potential (J)": _r(pe, 2)}

    @staticmethod
    def sound_freq(f, m):
        f = float(f)
        speeds = {"air": 343, "water": 1480, "iron": 5120}
        v = speeds.get(m.lower(), 343)
        return {"Wavelength (m)": _r(v/f, 3), "Velocity": v}

class Physics_Classes_11_12:
    TITLE = "Senior Physics: Advanced Labs"
    EXP_DATA = {
        "Vernier/Screw Gauge": ("precision", [("Tool (Vernier/Screw)", "Vernier"), ("Main Scale", "10"), ("VSD/Circular", "5"), ("LC", "0.01")]),
        "Projectile Motion": ("projectile", [("Vel (m/s)", "20"), ("Angle (deg)", "45")]),
        "Metre Bridge": ("meter_bridge", [("Standard R (Ω)", "10"), ("Balancing Length l (cm)", "40")]),
        "Potentiometer (Internal r)": ("pot_ir", [("L1 (open)", "100"), ("L2 (closed)", "60"), ("External R", "10")]),
        "Prism (Min Deviation)": ("prism", [("Angle A", "60"), ("Angle of Min Dev Dm", "30")]),
        "Young's Double Slit": ("ydse", [("Wave (nm)", "600"), ("Slit d (mm)", "0.5"), ("Dist D (m)", "1")]),
        "Brewster's Angle": ("brewster", [("Refractive Index n", "1.5")]),
        "Logic Gate Simulator": ("logic", [("Gate (AND/OR/XOR/NAND/NOR)", "AND"), ("A (1/0)", "1"), ("B (1/0)", "0")]),
        "Radioactive Half-life": ("decay", [("N0 (Initial)", "100"), ("T-half (s)", "60"), ("Time (s)", "120")]),
        "Newton's Cooling": ("cooling", [("Ambient Ta", "25"), ("Initial T0", "100"), ("k", "0.1"), ("Time", "10")]),
    }

    @staticmethod
    def precision(t, ms, sd, lc):
        ms, sd, lc = float(ms), float(sd), float(lc)
        return {"Total Reading": _r(ms + (sd * lc), 3)}

    @staticmethod
    def projectile(v, theta):
        v, theta = float(v), float(theta)
        rad = math.radians(theta)
        g = 9.8
        rng = (v**2 * math.sin(2*rad)) / g
        hmax = (v**2 * math.sin(rad)**2) / (2*g)
        return {"Range (m)": _r(rng, 2), "Height (m)": _r(hmax, 2)}

    @staticmethod
    def meter_bridge(r, l):
        r, l = float(r), float(l)
        x = r * (100 - l) / l
        return {"Unknown X (Ω)": _r(x, 2)}

    @staticmethod
    def pot_ir(l1, l2, r):
        l1, l2, r = float(l1), float(l2), float(r)
        # r = R * (l1 - l2) / l2
        ir = r * (l1 - l2) / l2
        return {"Internal r (Ω)": _r(ir, 3)}

    @staticmethod
    def prism(a, dm):
        a, dm = math.radians(float(a)), math.radians(float(dm))
        n = math.sin((a + dm) / 2) / math.sin(a / 2)
        return {"Refractive Index n": _r(n, 3)}

    @staticmethod
    def ydse(lam, d, big_d):
        beta = (float(lam)*1e-9 * float(big_d)) / (float(d)*1e-3)
        return {"Fringe Width (m)": f"{beta:.4e}"}

    @staticmethod
    def brewster(n):
        return {"Polarizing Angle (deg)": _r(math.degrees(math.atan(float(n))), 2)}

    @staticmethod
    def logic(g, a, b):
        a, b = int(a), int(b)
        mode = g.upper()
        if mode == "AND": res = a and b
        elif mode == "OR": res = a or b
        elif mode == "XOR": res = a != b
        elif mode == "NAND": res = not (a and b)
        elif mode == "NOR": res = not (a or b)
        else: res = 0
        return {"Output": int(res)}

    @staticmethod
    def decay(n0, th, t):
        n0, th, t = float(n0), float(th), float(t)
        nt = n0 * (0.5 ** (t / th))
        return {"Remaining N(t)": _r(nt, 2)}

    @staticmethod
    def cooling(ta, t0, k, t):
        ta, t0, k, t = float(ta), float(t0), float(k), float(t)
        tf = ta + (t0 - ta) * math.exp(-k * t)
        return {"Temp at time t (C)": _r(tf, 2)}

PHYSICS_REGISTRY = {
    "Classes 6-10": Physics_Classes_6_10,
    "Classes 11-12": Physics_Classes_11_12,
}
