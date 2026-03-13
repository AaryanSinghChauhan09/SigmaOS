"""
SigmaOS NCERT Physics Lab v10.0 — The Ultimate Series
Classes 6–12 | Exhaustive NCERT Experiment & Simulation Hub
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Physics_Classes_6_10:
    TITLE = "Secondary Physics: Comprehensive Foundations"
    EXP_DATA = {
        "Magnet Interaction": ("magnet", [("P1 (N/S)", "N"), ("P2 (N/S)", "S")]),
        "Light & Shadow": ("shadow", [("Dist (cm)", "20"), ("Height (cm)", "50")]),
        "Ohm's Law (V=IR)": ("ohms", [("V (V)", "10"), ("R (Ω)", "5")]),
        "Mirror/Lens Calc": ("optics", [("Type (M/L)", "Mirror"), ("u (cm)", "-20"), ("f (cm)", "10")]),
        "Archimedes/Density": ("buoyancy", [("Vol (m³)", "0.001"), ("Rho (kg/m³)", "1000")]),
        "Joule's Heating": ("joules", [("I (A)", "2"), ("R (Ω)", "10"), ("Time (s)", "60")]),
        "Snell's Law": ("snell", [("n1", "1"), ("Angle1 (deg)", "30"), ("n2", "1.5")]),
        "Gravity Logic": ("gravity", [("Mass (kg)", "60"), ("Body", "Earth")]),
        "Pendulum Clock": ("pendulum", [("Length (m)", "1")]),
        "Sound Speed (fλ)": ("sound", [("Freq (Hz)", "440"), ("Lambda (m)", "0.78")]),
    }

    @staticmethod
    def magnet(p1, p2):
        return {"Result": "REPEL" if p1.upper()==p2.upper() else "ATTRACT"}

    @staticmethod
    def shadow(d, h):
        return {"Scale": _r(h/d, 2)}

    @staticmethod
    def ohms(v, r):
        return {"I (A)": _r(v/r), "P (W)": _r(v**2/r)}

    @staticmethod
    def optics(t, u, f):
        if "mirror" in t.lower(): v = 1/(1/f - 1/u)
        else: v = 1/(1/f + 1/u)
        return {"v (cm)": _r(v, 2), "Mag": _r(-v/u if "mirror" in t.lower() else v/u, 2)}

    @staticmethod
    def buoyancy(v, d):
        return {"Upthrust (N)": _r(v*d*9.81, 2)}

    @staticmethod
    def joules(i, r, t):
        h = i**2 * r * t
        return {"Heat (J)": _r(h, 1), "In Cal": _r(h/4.184, 1)}

    @staticmethod
    def snell(n1, th, n2):
        r1 = math.radians(th)
        s2 = (n1 * math.sin(r1)) / n2
        if s2 > 1: return {"Result": "TIR (Total Internal Ref)"}
        return {"r Angle (deg)": _r(math.degrees(math.asin(s2)), 2)}

    @staticmethod
    def gravity(m, b):
        g = {"earth":9.81, "moon":1.62, "mars":3.71, "jupiter":24.79}.get(b.lower(), 9.81)
        return {"Weight (N)": _r(m*g, 2)}

    @staticmethod
    def pendulum(l):
        return {"Period T (s)": _r(2 * math.pi * math.sqrt(l/9.81), 3)}

    @staticmethod
    def sound(f, l):
        return {"v (m/s)": _r(f*l, 1)}

class Physics_Classes_11_12:
    TITLE = "Senior Physics: Advanced Laboratory Suite"
    EXP_DATA = {
        "Coulomb Force": ("coulomb", [("q1 (uC)", "10"), ("q2 (uC)", "-5"), ("r (cm)", "3")]),
        "Doppler Shift": ("doppler", [("Freq (Hz)", "500"), ("Vs (m/s)", "20"), ("Vo (m/s)", "10")]),
        "Kirchhoff's Node": ("kirch_node", [("I1 (A)", "5"), ("I2 (A)", "-2"), ("I3 (A)", "-1")]),
        "Boyle's Law": ("boyle", [("P1 (atm)", "1"), ("V1 (L)", "22.4"), ("V2 (L)", "11.2")]),
        "YDSE (Fringe)": ("ydse", [("Lambda (nm)", "589"), ("d (mm)", "0.1"), ("D (m)", "1")]),
        "Photoelectric": ("photo", [("Wavelength (nm)", "400"), ("WorkFn (eV)", "2.3")]),
        "Viscosity (Stokes)": ("visco", [("Radius (mm)", "1"), ("Density_s", "7800"), ("Density_f", "1260"), ("vTerm", "0.5")]),
        "Specific Heat": ("spec_heat", [("Mass (kg)", "1"), ("dT (K)", "10")]),
        "Biot-Savart (Loop)": ("biot_loop", [("Current (A)", "5"), ("Radius (cm)", "10"), ("Dist z (cm)", "0")]),
        "Radioactive Half": ("decay", [("N0", "1000"), ("T_half (s)", "60"), ("Time (s)", "180")]),
    }

    @staticmethod
    def coulomb(q1, q2, r):
        k = 9e9; q1 *= 1e-6; q2 *= 1e-6; r /= 100
        f = k * (q1 * q2) / r**2
        return {"Force (N)": _r(f, 3), "Nature": "ATTR" if f<0 else "REPEL"}

    @staticmethod
    def doppler(f, vs, vo):
        v = 343
        fd = f * (v + vo) / (v - vs)
        return {"Observed F (Hz)": _r(fd, 2)}

    @staticmethod
    def kirch_node(i1, i2, i3):
        res = i1 + i2 + i3
        return {"Sum outgoing": res, "Valid": _r(res)==0}

    @staticmethod
    def boyle(p1, v1, v2):
        return {"P2 (atm)": _r(p1*v1/v2, 3)}

    @staticmethod
    def ydse(lam, d, big_d):
        w = (lam*1e-9 * big_d) / (d*1e-3)
        return {"Width (mm)": _r(w*1000, 3)}

    @staticmethod
    def photo(lam, phi):
        e = 1240/lam
        return {"E_ph (eV)": _r(e, 2), "Emission": e > phi}

    @staticmethod
    def visco(r, rs, rf, v):
        r /= 1000
        eta = (2 * r**2 * (rs-rf) * 9.81) / (9 * v)
        return {"Viscosity (Pa.s)": _r(eta, 4)}

    @staticmethod
    def spec_heat(m, dt):
        return {"Energy (J)": _r(m * 4184 * dt, 1)}

    @staticmethod
    def biot_loop(i, r, z):
        mu0 = 4 * math.pi * 1e-7; r /= 100; z /= 100
        bz = (mu0 * i * r**2) / (2 * (r**2 + z**2)**1.5)
        return {"B (Tesla)": f"{bz:.4e}"}

    @staticmethod
    def decay(n0, th, t):
        nt = n0 * (0.5**(t/th))
        return {"Remaining": _r(nt, 2), "Decayed": _r(n0-nt, 2)}

PHYSICS_REGISTRY = {
    "Classes 6-10": Physics_Classes_6_10,
    "Classes 11-12": Physics_Classes_11_12,
}
