"""
SigmaOS NCERT Physics Lab v4.0
Classes 6–12 | Every NCERT Physics experiment & calculation
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Physics_Classes_6_10:
    TITLE = "Classes 6–10 – Light, Motion, Electricity & Gravitation"
    EXP_DATA = {
        "Ohm's Law": ("ohms_law", [("Voltage V (V)", "10"), ("Resistance R (Ω)", "5")]),
        "Law of Reflection": ("reflection", [("Angle of Incidence (°)", "45")]),
        "Acceleration": ("accel", [("Final Vel (m/s)", "20"), ("Initial Vel (m/s)", "0"), ("Time (s)", "5")]),
        "Force (F=ma)": ("force", [("Mass (kg)", "10"), ("Accel (m/s²)", "2")]),
        "Work Done": ("work", [("Force (N)", "50"), ("Distance (m)", "10")]),
        "Lens Formula": ("lens", [("Object dist u (cm)", "-20"), ("Focal length f (cm)", "10")]),
        "Mirror Formula": ("mirror", [("Object dist u (cm)", "-30"), ("Focal length f (cm)", "-15")]),
        "Resistance Combinations": ("res_comb", [("R1 (Ω)", "10"), ("R2 (Ω)", "20"), ("Type (s/p)", "p")]),
        "Universal Gravitation": ("gravity", [("Mass 1 (kg)", "5.97e24"), ("Mass 2 (kg)", "7.35e22"), ("Distance (m)", "3.84e8")]),
        "Sound Velocity": ("sound", [("Frequency (Hz)", "440"), ("Wavelength (m)", "0.78")]),
        "Density Calculator": ("density", [("Mass (kg)", "10"), ("Volume (m³)", "0.002")]),
    }

    @staticmethod
    def ohms_law(V, R):
        return {"Current I (A)": _r(V/R, 4), "Power P (W)": _r(V**2/R, 2)}

    @staticmethod
    def reflection(angle_i):
        return {"Angle of Reflection": angle_i, "Note": "Incident ray, reflected ray and normal lie in same plane"}

    @staticmethod
    def accel(v, u, t):
        return {"Acceleration (m/s²)": _r((v-u)/t, 2)}

    @staticmethod
    def force(m, a):
        return {"Force (N)": m * a}

    @staticmethod
    def work(f, d):
        return {"Work (Joules)": f * d}

    @staticmethod
    def lens(u, f):
        v_inv = 1/f + 1/u
        v = 1/v_inv
        m = v / u
        return {"Image dist v (cm)": _r(v, 2), "Magnification m": _r(m, 2)}

    @staticmethod
    def mirror(u, f):
        v_inv = 1/f - 1/u
        v = 1/v_inv
        m = -v / u
        return {"Image dist v (cm)": _r(v, 2), "Magnification m": _r(m, 2)}

    @staticmethod
    def res_comb(r1, r2, mode):
        if mode.lower() == "s": return {"Req (Series)": r1 + r2}
        return {"Req (Parallel)": _r((r1*r2)/(r1+r2), 2)}

    @staticmethod
    def gravity(m1, m2, r):
        G = 6.674e-11
        f = G * m1 * m2 / (r**2)
        return {"Force (N)": f"{f:.4e}"}

    @staticmethod
    def sound(freq, lam):
        return {"Velocity (m/s)": freq * lam}

    @staticmethod
    def density(m, v):
        return {"Density (kg/m³)": m / v}

class Physics_Classes_11_12:
    TITLE = "Classes 11–12 – Mechanics, Thermodynamics, Electromagnetism & Modern Physics"
    EXP_DATA = {
        "Vernier Caliper": ("vernier", [("Main Scale (cm)", "2.5"), ("Vernier Div", "4"), ("LC (cm)", "0.01")]),
        "Screw Gauge": ("screw", [("Pitch Scale (mm)", "5"), ("Head Div", "32"), ("LC (mm)", "0.01")]),
        "Moment of Inertia": ("moi", [("Shape (Ring/Disc/Sphere)", "Ring"), ("Mass (kg)", "2"), ("Radius (m)", "0.5")]),
        "Escape Velocity": ("escape_vel", [("Planet Mass (kg)", "5.97e24"), ("Radius (m)", "6.37e6")]),
        "Carnot Efficiency": ("carnot", [("Source Temp (K)", "600"), ("Sink Temp (K)", "300")]),
        "Projectile Motion": ("projectile", [("Velocity (m/s)", "20"), ("Angle (deg)", "45")]),
        "Young's Double Slit": ("ydse", [("Wavelength (nm)", "600"), ("Slit dist d (mm)", "0.5"), ("Screen dist D (m)", "1")]),
        "Wheatstone Bridge": ("wheatstone", [("P (Ω)", "10"), ("Q (Ω)", "20"), ("R (Ω)", "30")]),
        "Lorentz Force": ("lorentz", [("Charge (C)", "1.6e-19"), ("Velocity (m/s)", "1e5"), ("B Field (T)", "0.5")]),
        "Half Life Decay": ("halflife", [("Initial Amt", "100"), ("Half-life (s)", "60"), ("Elapsed (s)", "120")]),
        "Logic Gates": ("logic", [("Gate (AND/OR/XOR/NAND/NOR)", "AND"), ("A (1/0)", "1"), ("B (1/0)", "0")]),
    }

    @staticmethod
    def vernier(msr, vsd, lc):
        return {"Total Reading (cm)": _r(msr + (vsd * lc), 3)}

    @staticmethod
    def screw(psr, hsd, lc):
        return {"Total Reading (mm)": _r(psr + (hsd * lc), 3)}

    @staticmethod
    def moi(shape, m, r):
        s = shape.lower()
        if "ring" in s: i = m * r**2
        elif "disc" in s: i = 0.5 * m * r**2
        elif "sphere" in s: i = 0.4 * m * r**2 # Solid
        else: return {"Error": "Shape unknown"}
        return {"MOI (kg·m²)": _r(i, 4)}

    @staticmethod
    def escape_vel(m, r):
        G = 6.674e-11
        v = math.sqrt(2 * G * m / r)
        return {"Escape Velocity (m/s)": _r(v, 2)}

    @staticmethod
    def carnot(th, tc):
        eff = 1 - (tc / th)
        return {"Efficiency": _r(eff, 4), "Percentage": f"{_r(eff*100, 2)}%"}

    @staticmethod
    def projectile(v, theta_deg):
        g = 9.8
        tr = math.radians(theta_deg)
        r = (v**2 * math.sin(2 * tr)) / g
        h = (v**2 * (math.sin(tr)**2)) / (2 * g)
        t = (2 * v * math.sin(tr)) / g
        return {"Range (m)": _r(r, 2), "Max Height (m)": _r(h, 2), "Time (s)": _r(t, 2)}

    @staticmethod
    def ydse(lam_nm, d_mm, big_d):
        lam = lam_nm * 1e-9
        small_d = d_mm * 1e-3
        width = (lam * big_d) / small_d
        return {"Fringe Width (m)": f"{width:.4e}"}

    @staticmethod
    def wheatstone(p, q, r):
        # S = (Q/P) * R
        s = (q / p) * r
        return {"Unknown Resistance S (Ω)": _r(s, 2)}

    @staticmethod
    def lorentz(q, v, b):
        # F = qvB (max)
        f = q * v * b
        return {"Magnetic Force (N)": f"{f:.4e}"}

    @staticmethod
    def logic(gate, a, b):
        g = gate.upper()
        if g == "AND": out = 1 if a and b else 0
        elif g == "OR": out = 1 if a or b else 0
        elif g == "XOR": out = 1 if a != b else 0
        elif g == "NAND": out = 0 if a and b else 1
        elif g == "NOR": out = 1 if not (a or b) else 0
        else: return {"Error": "Invalid"}
        return {"Result": out}

    @staticmethod
    def halflife(n0, th, t):
        nt = n0 * (0.5 ** (t / th))
        return {"Remaining": _r(nt, 4)}

PHYSICS_REGISTRY = {
    "Classes 6-10": Physics_Classes_6_10,
    "Classes 11-12": Physics_Classes_11_12,
}
