"""
SigmaOS NCERT Physics Lab v7.0 — The Ultimate Lab Manual
Classes 6–12 | Every Core NCERT Experiment & Calculator
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Physics_Classes_6_10:
    TITLE = "Secondary Physics: Foundations of Science"
    EXP_DATA = {
        "Ohm's Law": ("ohms_law", [("Voltage (V)", "10"), ("Resistance (Ω)", "5")]),
        "Mirror/Lens Formula": ("optics", [("Type (Mirror/Lens)", "Mirror"), ("Object dist u (cm)", "-20"), ("Focal length f (cm)", "10")]),
        "Resistance Combinations": ("resistors", [("R1", "10"), ("R2", "20"), ("R3", "30"), ("Type (Series/Para)", "Series")]),
        "Gravitation (Weight)": ("gravity_weight", [("Mass (kg)", "60"), ("Planet (Earth/Moon/Mars)", "Earth")]),
        "Archimedes Principle": ("buoyancy", [("Object Vol (m³)", "0.001"), ("Fluid Density (kg/m³)", "1000")]),
        "Magnetic Field (Wire)": ("mag_field", [("Current (A)", "5"), ("Distance (m)", "0.02")]),
        "Work & Energy": ("energy", [("Mass (kg)", "2"), ("Velocity (m/s)", "10"), ("Height (m)", "5")]),
        "Atmospheric Pressure": ("atm_p", [("Altitude (m)", "0")]),
        "Law of Flotation": ("float", [("Object Density", "800"), ("Fluid Density", "1000")]),
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
        gs = {"earth": 9.8, "moon": 1.62, "mars": 3.71, "jupiter": 24.79}
        g = gs.get(p.lower(), 9.8)
        return {"Weight (N)": _r(float(m)*g, 1), "g (m/s²)": g}

    @staticmethod
    def buoyancy(v, d):
        v, d = float(v), float(d)
        f = v * d * 9.8
        return {"Upthrust (N)": _r(f, 2), "Note": "Buoyant force acts upwards"}

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
    def atm_p(h):
        h = float(h)
        p = 101325 * (1 - 2.25577e-5 * h)**5.25588
        return {"Pressure (Pa)": _r(p, 0), "In atm": _r(p/101325, 3)}

    @staticmethod
    def float(od, fd):
        od, fd = float(od), float(fd)
        if od < fd: return {"Result": "FLOTATION", "Note": "Object will float (Density < Fluid)"}
        return {"Result": "SINKING", "Note": "Object will sink (Density >= Fluid)"}

class Physics_Classes_11_12:
    TITLE = "Senior Physics: Advanced Laboratory"
    EXP_DATA = {
        "Screw Gauge/Vernier": ("precision", [("Tool (Vernier/Screw)", "Vernier"), ("Main Scale", "10"), ("VSD/Circular", "5"), ("LC", "0.01")]),
        "Viscosity (Stoke's Law)": ("viscosity", [("Ball Radius (mm)", "1"), ("Density (kg/m³)", "7800"), ("Fluid Density", "1260"), ("Term. Vel (m/s)", "0.5")]),
        "Newton's Cooling": ("cooling", [("Ambient Temp", "25"), ("Initial Temp", "100"), ("Time (mins)", "10"), ("k constant", "0.1")]),
        "Parallelogram Law": ("parallel_forces", [("Force P (N)", "3"), ("Force Q (N)", "4"), ("Angle (deg)", "90")]),
        "Meter Bridge (Resistivity)": ("meter_bridge", [("Standard R", "10"), ("Balancing Length l (cm)", "40")]),
        "Potentiometer (EMF)": ("potentiometer", [("Standard E1 (V)", "1.5"), ("l1 (cm)", "60"), ("l2 (cm)", "80")]),
        "Zener Diode": ("zener", [("Input V (V)", "10"), ("Zener Vz (V)", "5"), ("Resistor Rs (Ω)", "100")]),
        "PN Junction (Forward)": ("pn_junction", [("Voltage V (V)", "0.7"), ("Temp (K)", "300")]),
        "Brewster's Angle": ("brewster", [("Refractive Index n", "1.5")]),
        "Refractive Index (Prism)": ("prism", [("Angle A", "60"), ("Angle of Min Dev Dm", "30")]),
        "Photoelectric Theory": ("photo", [("Work Function (eV)", "2.1"), ("Wavelength (nm)", "400")]),
    }

    @staticmethod
    def precision(t, ms, sd, lc):
        ms, sd, lc = float(ms), float(sd), float(lc)
        return {"Total Reading": _r(ms + (sd * lc), 3)}

    @staticmethod
    def viscosity(r, ds, df, v):
        r, ds, df, v = float(r)/1000, float(ds), float(df), float(v)
        # eta = 2r^2(ds-df)g / 9v
        eta = (2 * r**2 * (ds - df) * 9.8) / (9 * v)
        return {"Viscosity (Pa·s)": _r(eta, 4)}

    @staticmethod
    def cooling(ta, t0, t, k):
        ta, t0, t, k = float(ta), float(t0), float(t), float(k)
        # T(t) = Ta + (T0-Ta)e^-kt
        tf = ta + (t0 - ta) * math.exp(-k * t)
        return {"Final Temp (C)": _r(tf, 2)}

    @staticmethod
    def parallel_forces(p, q, theta):
        p, q, theta = float(p), float(q), math.radians(float(theta))
        r = math.sqrt(p**2 + q**2 + 2*p*q*math.cos(theta))
        return {"Resultant Force (N)": _r(r, 2)}

    @staticmethod
    def meter_bridge(r, l):
        r, l = float(r), float(l)
        # X = R * (100-l)/l
        x = r * (100 - l) / l
        return {"Unknown Resistance X (Ω)": _r(x, 2)}

    @staticmethod
    def potentiometer(e1, l1, l2):
        e1, l1, l2 = float(e1), float(l1), float(l2)
        # E2 = E1 * (l2/l1)
        e2 = e1 * (l2 / l1)
        return {"Unknown EMF E2 (V)": _r(e2, 3)}

    @staticmethod
    def zener(vi, vz, rs):
        vi, vz, rs = float(vi), float(vz), float(rs)
        if vi < vz: return {"Status": "Regulator NOT Active", "Output V": vi}
        is_val = (vi - vz) / rs
        return {"Status": "Regulator ACTIVE", "Output V": vz, "Current Is (A)": _r(is_val, 3)}

    @staticmethod
    def pn_junction(v, t):
        v, t = float(v), float(t)
        # I = Is(e^V/nkT - 1) simplified
        k = 1.38e-23; q = 1.6e-19
        i_ratio = math.exp((q * v) / (k * t))
        return {"Current Factor (e^V/Vt)": f"{i_ratio:.4e}", "Note": "Current rises exponentially in Forward Bias"}

    @staticmethod
    def brewster(n):
        n = float(n)
        angle = math.degrees(math.atan(n))
        return {"Polarizing Angle (deg)": _r(angle, 2)}

    @staticmethod
    def prism(a, dm):
        a, dm = math.radians(float(a)), math.radians(float(dm))
        # n = sin((A+Dm)/2) / sin(A/2)
        n = math.sin((a + dm) / 2) / math.sin(a / 2)
        return {"Refractive Index n": _r(n, 3)}

    @staticmethod
    def photo(phi, lam):
        phi, lam = float(phi), float(lam)
        h = 6.626e-34; c = 3e8; ev = 1.6e-19
        e_in = (h * c) / (lam * 1e-9 * ev)
        kmax = e_in - phi
        return {"Photon Energy (eV)": _r(e_in, 2), "K-max (eV)": _r(max(0, kmax), 2), "Emitted": "YES" if kmax > 0 else "NO"}

PHYSICS_REGISTRY = {
    "Classes 6-10": Physics_Classes_6_10,
    "Classes 11-12": Physics_Classes_11_12,
}
