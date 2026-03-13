"""
SigmaOS NCERT Physics Lab v6.0 — The Ultimate Series
Classes 6–12 | Every Core NCERT Experiment & Calculator
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Physics_Classes_6_10:
    TITLE = "Secondary Physics: Light, Motion, Energy & Magnetism"
    EXP_DATA = {
        "Ohm's Law": ("ohms_law", [("Voltage (V)", "10"), ("Resistance (Ω)", "5")]),
        "Mirror/Lens Formula": ("optics", [("Type (Mirror/Lens)", "Mirror"), ("Object dist u (cm)", "-20"), ("Focal length f (cm)", "10")]),
        "Resistance Combinations": ("resistors", [("R1", "10"), ("R2", "20"), ("R3", "30"), ("Type (Series/Para)", "Series")]),
        "Gravitation (Weight)": ("gravity_weight", [("Mass (kg)", "60"), ("Planet (Earth/Moon/Mars)", "Earth")]),
        "Archimedes Principle": ("buoyancy", [("Object Vol (m³)", "0.001"), ("Fluid Density (kg/m³)", "1000")]),
        "Magnetic Field (Wire)": ("mag_field", [("Current (A)", "5"), ("Distance (m)", "0.02")]),
        "Work & Energy": ("energy", [("Mass (kg)", "2"), ("Velocity (m/s)", "10"), ("Height (m)", "5")]),
        "Sound Waves": ("sound", [("Frequency (Hz)", "440"), ("Wavelength (m)", "0.78")]),
        "Heat Capacity": ("heat", [("Mass (kg)", "0.5"), ("Sp. Heat (J/kgK)", "4184"), ("dT (K)", "10")]),
    }

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
        return {"Image dist v (cm)": _r(v, 2), "Magnification m": _r(m, 2)}

    @staticmethod
    def resistors(r1, r2, r3, mode):
        r1, r2, r3 = float(r1), float(r2), float(r3)
        if "series" in mode.lower(): 
            req = r1 + r2 + r3
        else: 
            req = 1 / (1/r1 + 1/r2 + 1/r3)
        return {"Req (Total)": _r(req, 2)}

    @staticmethod
    def gravity_weight(m, p):
        gs = {"earth": 9.8, "moon": 1.62, "mars": 3.71}
        g = gs.get(p.lower(), 9.8)
        return {"Weight (N)": _r(float(m)*g, 1), "g (m/s²)": g}

    @staticmethod
    def buoyancy(v, d):
        v, d = float(v), float(d)
        f = v * d * 9.8
        return {"Upthrust (N)": _r(f, 2), "Note": "Object will float if Density < Fluid Density"}

    @staticmethod
    def mag_field(i, r):
        i, r = float(i), float(r)
        b = (4 * math.pi * 1e-7 * i) / (2 * math.pi * r)
        return {"B Field (Tesla)": f"{b:.4e}"}

    @staticmethod
    def energy(m, v, h):
        m, v, h = float(m), float(v), float(h)
        ke = 0.5 * m * v**2
        pe = m * 9.8 * h
        return {"Kinetic (J)": _r(ke, 2), "Potential (J)": _r(pe, 2)}

    @staticmethod
    def sound(f, l):
        f, l = float(f), float(l)
        return {"Velocity (m/s)": _r(f*l, 1), "Observation": "Normal air speed ~340m/s"}

    @staticmethod
    def heat(m, c, dt):
        m, c, dt = float(m), float(c), float(dt)
        return {"Heat Q (J)": _r(m * c * dt, 1)}

class Physics_Classes_11_12:
    TITLE = "Senior Physics: Advanced Mechanics, E-Mag, Waves & Modern Physics"
    EXP_DATA = {
        "Vernier/Screw Gauge": ("precision", [("Tool (Vernier/Screw)", "Vernier"), ("Main Scale", "10"), ("Circular/VSD", "5"), ("LC", "0.01")]),
        "Young's Modulus": ("young", [("Load (kg)", "5"), ("Length (m)", "2"), ("Radius (mm)", "0.5"), ("dl (mm)", "1")]),
        "Surface Tension": ("stension", [("Force (N)", "0.07"), ("Length (m)", "1")]),
        "Projectile Motion": ("projectile", [("Vel (m/s)", "20"), ("Angle (deg)", "45")]),
        "Capillary Rise": ("capillary", [("Surface Tension", "0.07"), ("Radius (mm)", "0.5"), ("Density", "1000")]),
        "AC Filter (LCR)": ("lcr", [("R (Ω)", "10"), ("L (mH)", "50"), ("C (µF)", "100"), ("Freq (Hz)", "50")]),
        "Photoelectric Theory": ("photo", [("Work Function (eV)", "2.1"), ("Wavelength (nm)", "400")]),
        "Radioactive Decay": ("decay", [("Initial N0", "100"), ("Half-life (s)", "60"), ("Time (s)", "120")]),
        "Wheatstone Bridge": ("wheatstone", [("P (Ω)", "10"), ("Q (Ω)", "20"), ("R (Ω)", "30")]),
        "Logic Gates": ("logic", [("Gate (AND/OR/XOR/NAND)", "AND"), ("A (1/0)", "1"), ("B (1/0)", "0")]),
    }

    @staticmethod
    def precision(t, ms, sd, lc):
        ms, sd, lc = float(ms), float(sd), float(lc)
        return {"Total Reading": _r(ms + (sd * lc), 3)}

    @staticmethod
    def young(m, l, r, dl):
        m, l, r, dl = float(m), float(l), float(r)/1000, float(dl)/1000
        stress = (m * 9.8) / (math.pi * r**2)
        strain = dl / l
        return {"Y (N/m²)": f"{stress/strain:.4e}"}

    @staticmethod
    def stension(f, l):
        f, l = float(f), float(l)
        return {"T (N/m)": _r(f/l, 4)}

    @staticmethod
    def projectile(v, theta):
        v, theta = float(v), float(theta)
        rad = math.radians(theta)
        g = 9.8
        rng = (v**2 * math.sin(2*rad)) / g
        hmax = (v**2 * math.sin(rad)**2) / (2*g)
        t_f = (2*v*math.sin(rad))/g
        return {"Range (m)": _r(rng, 2), "Height (m)": _r(hmax, 2), "Time (s)": _r(t_f, 2)}

    @staticmethod
    def capillary(s, r, d):
        s, r, d = float(s), float(r)/1000, float(d)
        h = (2 * s) / (r * d * 9.8)
        return {"Height (m)": _r(h, 4)}

    @staticmethod
    def lcr(r, l, c, f):
        r, l, c, f = float(r), float(l), float(c), float(f)
        xl = 2 * math.pi * f * (l/1000)
        xc = 1 / (2 * math.pi * f * (c/1e6))
        z = math.sqrt(r**2 + (xl-xc)**2)
        return {"Impedance Z (Ω)": _r(z, 2), "Resonant Freq (Hz)": _r(1/(2*math.pi*math.sqrt(l/1000 * c/1e6)), 1)}

    @staticmethod
    def photo(phi, lam):
        phi, lam = float(phi), float(lam)
        h = 6.626e-34; c = 3e8; ev = 1.6e-19
        e_in = (h * c) / (lam * 1e-9 * ev)
        kmax = e_in - phi
        return {"Photon Energy (eV)": _r(e_in, 2), "K-max (eV)": _r(max(0, kmax), 2), "Emitted": "YES" if kmax > 0 else "NO"}

    @staticmethod
    def decay(n0, th, t):
        n0, th, t = float(n0), float(th), float(t)
        nt = n0 * (0.5 ** (t / th))
        return {"Remaining N(t)": _r(nt, 2), "Decayed": _r(n0-nt, 2)}

    @staticmethod
    def wheatstone(p, q, r):
        p, q, r = float(p), float(q), float(r)
        # P/Q = R/S => S = Q*R / P
        return {"Unknown S (Ω)": _r((q*r)/p, 2)}

    @staticmethod
    def logic(g, a, b):
        a, b = int(a), int(b)
        mode = g.upper()
        if mode == "AND": res = a and b
        elif mode == "OR": res = a or b
        elif mode == "XOR": res = a != b
        elif mode == "NAND": res = not (a and b)
        else: res = 0
        return {"Output": int(res)}

PHYSICS_REGISTRY = {
    "Classes 6-10": Physics_Classes_6_10,
    "Classes 11-12": Physics_Classes_11_12,
}
