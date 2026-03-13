"""
SigmaOS NCERT Physics Lab v5.0 — The Complete Series
Classes 6–12 | Comprehensive Experiment Repository
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Physics_Classes_6_10:
    TITLE = "Classes 6–10: Mechanics, Light, Electricity & Magnetism"
    EXP_DATA = {
        "Ohm's Law": ("ohms_law", [("Voltage (V)", "10"), ("Resistance (Ohm)", "5")]),
        "Mirror/Lens Formula": ("optics", [("Type (Mirror/Lens)", "Mirror"), ("Object dist u (cm)", "-20"), ("Focal length f (cm)", "10")]),
        "Resistance Combinations": ("resistors", [("R1", "10"), ("R2", "20"), ("R3", "30"), ("Type (Series/Para)", "Series")]),
        "Atmospheric Pressure": ("pressure", [("Height above sea (m)", "1000")]),
        "Archimedes Principle": ("buoyancy", [("Object Vol (m3)", "0.001"), ("Fluid Density (kg/m3)", "1000")]),
        "Magnetic Field (Wire)": ("mag_field", [("Current (A)", "5"), ("Distance (m)", "0.02")]),
        "Kinetic/Potential Energy": ("energy", [("Mass (kg)", "2"), ("Velocity (m/s)", "10"), ("Height (m)", "5")]),
        "Simple Pendulum (Time)": ("pendulum", [("Length (m)", "1"), ("Gravity (m/s2)", "9.8")]),
    }

    @staticmethod
    def ohms_law(v, r):
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
        if "series" in mode.lower(): 
            req = r1 + r2 + r3
        else: 
            req = 1 / (1/r1 + 1/r2 + 1/r3)
        return {"Req (Total)": _r(req, 2)}

    @staticmethod
    def pressure(h):
        # P = P0 * exp(-mgh/kT) simplified
        p = 101325 * math.exp(-0.00012 * h)
        return {"Pressure (Pa)": _r(p, 0), "In atm": _r(p/101325, 3)}

    @staticmethod
    def buoyancy(v, d):
        f = v * d * 9.8
        return {"Upthrust (N)": _r(f, 2)}

    @staticmethod
    def mag_field(i, r):
        # B = mu0 * I / 2pi*r
        b = (4 * math.pi * 1e-7 * i) / (2 * math.pi * r)
        return {"B Field (Tesla)": f"{b:.4e}"}

    @staticmethod
    def energy(m, v, h):
        ke = 0.5 * m * v**2
        pe = m * 9.8 * h
        return {"Kinetic (J)": _r(ke, 2), "Potential (J)": _r(pe, 2), "Total (J)": _r(ke+pe, 2)}

    @staticmethod
    def pendulum(l, g):
        t = 2 * math.pi * math.sqrt(l/g)
        return {"Time Period (s)": _r(t, 2)}

class Physics_Classes_11_12:
    TITLE = "Classes 11–12: Advanced Mechanics, Waves, E-Mag & Modern Physics"
    EXP_DATA = {
        "Projectile Motion": ("projectile", [("Vel (m/s)", "20"), ("Angle (deg)", "45")]),
        "Young's Double Slit": ("ydse", [("Wave (nm)", "600"), ("Slit d (mm)", "0.5"), ("Dist D (m)", "1")]),
        "Bohr's Atom (Hydrogen)": ("bohr", [("Orbit n", "1")]),
        "AC Impedance (LCR)": ("lcr", [("R (ohm)", "10"), ("L (mH)", "50"), ("C (uF)", "100"), ("Freq (Hz)", "50")]),
        "Velocity of Sound (Resinence)": ("sound_vel", [("Freq (Hz)", "512"), ("L1 (cm)", "16.1"), ("L2 (cm)", "49.5")]),
        "Photoelectric Equation": ("photo", [("Work Function (eV)", "2.1"), ("Light Wave (nm)", "400")]),
        "Coulomb's Law": ("coulomb", [("q1 (C)", "1e-6"), ("q2 (C)", "1e-6"), ("r (m)", "0.1")]),
        "Einstein Mass-Energy": ("einstein", [("Mass (kg)", "0.001")]),
    }

    @staticmethod
    def projectile(v, theta):
        rad = math.radians(theta)
        g = 9.8
        rng = (v**2 * math.sin(2*rad)) / g
        hmax = (v**2 * math.sin(rad)**2) / (2*g)
        return {"Range (m)": _r(rng, 2), "Max Height (m)": _r(hmax, 2)}

    @staticmethod
    def ydse(lam, d, big_d):
        beta = (lam*1e-9 * big_d) / (d*1e-3)
        return {"Fringe Width (m)": f"{beta:.4e}"}

    @staticmethod
    def bohr(n):
        e = -13.6 / (n**2)
        r = 0.529 * (n**2)
        return {"Energy (eV)": _r(e, 2), "Radius (A)": _r(r, 3)}

    @staticmethod
    def lcr(r, l, c, f):
        xl = 2 * math.pi * f * (l/1000)
        xc = 1 / (2 * math.pi * f * (c/1e6))
        z = math.sqrt(r**2 + (xl-xc)**2)
        phi = math.degrees(math.atan((xl-xc)/r))
        return {"Impedance Z (ohm)": _r(z, 2), "Phase Angle": _r(phi, 2)}

    @staticmethod
    def sound_vel(f, l1, l2):
        v = 2 * f * ((l2-l1)/100)
        return {"Velocity (m/s)": _r(v, 2)}

    @staticmethod
    def photo(phi_ev, lam_nm):
        h = 6.626e-34; c = 3e8; ev = 1.6e-19
        e_in = (h * c) / (lam_nm * 1e-9 * ev)
        kmax = e_in - phi_ev
        return {"Input Energy (eV)": _r(e_in, 2), "K-Max (eV)": _r(max(0, kmax), 2)}

    @staticmethod
    def coulomb(q1, q2, r):
        k = 8.987e9
        f = k * q1 * q2 / r**2
        return {"Force (N)": f"{f:.4e}"}

    @staticmethod
    def einstein(m):
        e = m * (3e8)**2
        return {"Energy (Joules)": f"{e:.4e}"}

PHYSICS_REGISTRY = {
    "Classes 6-10": Physics_Classes_6_10,
    "Classes 11-12": Physics_Classes_11_12,
}
