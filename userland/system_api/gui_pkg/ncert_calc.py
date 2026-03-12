import tkinter as tk
from tkinter import ttk
import os
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED

class NcertCalcPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "NCERT MASTER CALCULATOR", "Physics, Chemistry, Math & Bio Equation Solver")
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        card = self.controller._card(main_panel, "NCERT Omni-Calculator")
        card.master.pack(pady=50)
        
        tk.Label(card, text="The NCERT Master Calculator provides pre-built formulas for the 11th and 12th grade syllabus.",
                 font=FONT_MED, bg=PAL["card"], fg=PAL["dim"], wraplength=400).pack(pady=20, padx=20)
        
        def _launch():
            self._generate_and_launch_html()
            self.controller._notify("NCERT Calc", "Browser-based calculator launched.", "OK")

        ttk.Button(card, text="🧮 Launch NCERT Master Calculator", command=_launch, style="Teal.TButton").pack(pady=20)

    def _generate_and_launch_html(self):
        html_content = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SigmaOS NCERT Master Calculator</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: #0A0A12; color: #F2F2F7; margin: 0; padding: 20px; }
        .container { max-width: 900px; margin: 0 auto; background-color: #11111E; padding: 20px; border-radius: 10px; box-shadow: 0 4px 15px rgba(0,0,0,0.5); }
        h1 { color: #5AC8FA; font-size: 26px; text-align: center; }
        h2 { color: #FFCC00; font-size: 20px; border-bottom: 1px solid #38383A; padding-bottom: 10px; margin-top: 30px; }
        .grid-2 { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin-top: 15px; }
        .card { background-color: #1C1C1E; padding: 15px; border-radius: 8px; border: 1px solid #38383A; }
        label { display: block; font-size: 14px; color: #8E8E93; margin-bottom: 5px; }
        input, select { width: 100%; box-sizing: border-box; padding: 10px; margin-bottom: 15px; background-color: #252529; color: white; border: 1px solid #5AC8FA; border-radius: 4px; font-family: monospace; }
        .btn { background-color: #5AC8FA; color: #0A0A12; padding: 10px; width: 100%; border: none; border-radius: 4px; cursor: pointer; font-weight: bold; font-size: 14px; transition: 0.2s; }
        .btn:hover { background-color: #4CD964; }
        .output-box { background-color: #0A0A12; padding: 15px; border-radius: 4px; font-family: 'Consolas', monospace; color: #4CD964; margin-top: 15px; border: 1px solid #4CD964; white-space: pre-wrap; font-size: 16px;}
        .formula { font-style: italic; color: #FF3B30; font-size: 12px; margin-top: -10px; margin-bottom: 10px;}
    </style>
</head>
<body>
    <div class="container">
        <h1>🧮 NCERT Master Formula Calculator</h1>
        <p style="text-align: center; color: #8E8E93;">Instant syllabus calculations for Physics, Chemistry, Math & Biology</p>
        
        <!-- PHYSICS SECTION -->
        <h2>🔭 Physics (Class 11 & 12)</h2>
        <div class="grid-2">
            <!-- Kinematics -->
            <div class="card">
                <h3 style="color: #5AC8FA; margin-top: 0;">Kinematics: Final Velocity</h3>
                <div class="formula">Formula: v = u + at</div>
                <label>Initial Velocity (u) [m/s]</label>
                <input type="number" id="phys-u" value="0">
                <label>Acceleration (a) [m/s²]</label>
                <input type="number" id="phys-a" value="9.8">
                <label>Time (t) [s]</label>
                <input type="number" id="phys-t" value="5">
                <button class="btn" onclick="calcKinematics()">Calculate v</button>
            </div>
            
            <!-- Optics -->
            <div class="card">
                <h3 style="color: #5AC8FA; margin-top: 0;">Optics: Lens Formula</h3>
                <div class="formula">Formula: 1/f = 1/v - 1/u</div>
                <label>Image Distance (v) [cm]</label>
                <input type="number" id="phys-v" value="20">
                <label>Object Distance (u) [cm]</label>
                <input type="number" id="phys-obj-u" value="-30">
                <button class="btn" onclick="calcOptics()">Calculate f</button>
            </div>
        </div>

        <!-- CHEMISTRY SECTION -->
        <h2>🧪 Chemistry (Class 11 & 12)</h2>
        <div class="grid-2">
            <!-- Molarity -->
            <div class="card">
                <h3 style="color: #4CD964; margin-top: 0;">Solutions: Molarity</h3>
                <div class="formula">Formula: M = (m / MW) / V(L)</div>
                <label>Mass of Solute (g)</label>
                <input type="number" id="chem-m" value="5.85">
                <label>Molar Mass of Solute (g/mol)</label>
                <input type="number" id="chem-mw" value="58.5">
                <label>Volume of Solution (L)</label>
                <input type="number" id="chem-vol" value="0.5">
                <button class="btn" style="background-color: #4CD964;" onclick="calcMolarity()">Calculate Molarity (M)</button>
            </div>
            
            <!-- Ideal Gas -->
            <div class="card">
                <h3 style="color: #4CD964; margin-top: 0;">States of Matter: Ideal Gas</h3>
                <div class="formula">Formula: PV = nRT</div>
                <label>Pressure (P) [atm]</label>
                <input type="number" id="chem-p" value="1">
                <label>Moles of Gas (n)</label>
                <input type="number" id="chem-n" value="2">
                <label>Temperature (T) [K]</label>
                <input type="number" id="chem-t" value="298">
                <button class="btn" style="background-color: #4CD964;" onclick="calcGasLaw()">Calculate Volume (V)</button>
            </div>
        </div>

        <!-- MATH SECTION -->
        <h2>📐 Mathematics (Class 11 & 12)</h2>
        <div class="grid-2">
            <!-- Quadratic -->
            <div class="card">
                <h3 style="color: #FF9F0A; margin-top: 0;">Algebra: Quadratic Formula</h3>
                <div class="formula">Formula: x = [-b ± √(b² - 4ac)] / 2a</div>
                <label>Coefficient a</label>
                <input type="number" id="math-a" value="1">
                <label>Coefficient b</label>
                <input type="number" id="math-b" value="-5">
                <label>Coefficient c</label>
                <input type="number" id="math-c" value="6">
                <button class="btn" style="background-color: #FF9F0A; color: white;" onclick="calcQuadratic()">Solve for x</button>
            </div>
            
            <!-- Prob/Stats -->
            <div class="card">
                <h3 style="color: #FF9F0A; margin-top: 0;">Statistics: Binomial Probability</h3>
                <div class="formula">Formula: P(x) = C(n,x) * p^x * q^(n-x)</div>
                <label>Number of trials (n)</label>
                <input type="number" id="math-n" value="10">
                <label>Success prob (p)</label>
                <input type="number" id="math-p" value="0.5">
                <label>Number of successes (x)</label>
                <input type="number" id="math-x" value="5">
                <button class="btn" style="background-color: #FF9F0A; color: white;" onclick="calcBinomial()">Calculate P(x)</button>
            </div>
        </div>
        
        <!-- BIO SECTION -->
        <h2>🧬 Biology (Class 11 & 12)</h2>
        <div class="grid-2">
            <!-- Genetics -->
            <div class="card">
                <h3 style="color: #BF5AF2; margin-top: 0;">Genetics: Hardy-Weinberg Eq</h3>
                <div class="formula">Formula: p² + 2pq + q² = 1</div>
                <label>Frequency of recessive allele (q)</label>
                <input type="number" id="bio-q" value="0.4" step="0.01" max="1" min="0">
                <button class="btn" style="background-color: #BF5AF2; color: white;" onclick="calcHardyWeinberg()">Calculate Frequencies</button>
            </div>
            
            <!-- Respiration -->
            <div class="card">
                <h3 style="color: #BF5AF2; margin-top: 0;">Physiology: Cardiac Output</h3>
                <div class="formula">Formula: CO = Heart Rate × Stroke Volume</div>
                <label>Heart Rate (bpm)</label>
                <input type="number" id="bio-hr" value="72">
                <label>Stroke Volume (mL/beat)</label>
                <input type="number" id="bio-sv" value="70">
                <button class="btn" style="background-color: #BF5AF2; color: white;" onclick="calcCardiacOutput()">Calculate CO</button>
            </div>
        </div>

        <div class="card" style="margin-top: 20px;">
            <h2>Master Result Output</h2>
            <div class="output-box" id="master-out">Awaiting input...</div>
        </div>
    </div>

    <script>
        const out = document.getElementById('master-out');

        // Physics
        function calcKinematics() {
            const u = parseFloat(document.getElementById('phys-u').value);
            const a = parseFloat(document.getElementById('phys-a').value);
            const t = parseFloat(document.getElementById('phys-t').value);
            const v = u + (a * t);
            out.innerText = `[PHYSICS] Final Velocity calculated.\\nv = ${u} + (${a} × ${t})\\nv = ${v} m/s`;
        }

        function calcOptics() {
            const v = parseFloat(document.getElementById('phys-v').value);
            const u = parseFloat(document.getElementById('phys-obj-u').value);
            if(v===0 || u===0) { out.innerText = "Error: Division by zero."; return; }
            const invF = (1/v) - (1/u);
            const f = 1/invF;
            out.innerText = `[PHYSICS] Lens Focal Length calculated.\\n1/f = 1/${v} - 1/(${u})\\nf = ${f.toFixed(2)} cm`;
        }

        // Chemistry
        function calcMolarity() {
            const m = parseFloat(document.getElementById('chem-m').value);
            const mw = parseFloat(document.getElementById('chem-mw').value);
            const vol = parseFloat(document.getElementById('chem-vol').value);
            if(mw===0 || vol===0) { out.innerText = "Error: Division by zero."; return; }
            const moles = m / mw;
            const mol = moles / vol;
            out.innerText = `[CHEMISTRY] Molarity calculated.\\nMoles = ${m} / ${mw} = ${moles.toFixed(4)} mol\\nM = ${moles.toFixed(4)} / ${vol} L\\nMolarity = ${mol.toFixed(4)} M`;
        }

        function calcGasLaw() {
            const p = parseFloat(document.getElementById('chem-p').value);
            const n = parseFloat(document.getElementById('chem-n').value);
            const t = parseFloat(document.getElementById('chem-t').value);
            const r = 0.0821; // L atm K-1 mol-1
            if(p===0) { out.innerText = "Error: Pressure cannot be zero."; return; }
            const v = (n * r * t) / p;
            out.innerText = `[CHEMISTRY] Ideal Gas Volume calculated.\\nV = (${n} × ${r} × ${t}) / ${p}\\nV = ${v.toFixed(3)} L`;
        }

        // Math
        function calcQuadratic() {
            const a = parseFloat(document.getElementById('math-a').value);
            const b = parseFloat(document.getElementById('math-b').value);
            const c = parseFloat(document.getElementById('math-c').value);
            if(a === 0) { out.innerText = "Error: 'a' cannot be zero in a quadratic equation."; return; }
            
            const D = (b*b) - (4*a*c);
            let res = `[MATHEMATICS] Quadratic Roots calculated.\\nDiscriminant (D) = ${D}\\n`;
            
            if(D > 0) {
                const x1 = (-b + Math.sqrt(D)) / (2*a);
                const x2 = (-b - Math.sqrt(D)) / (2*a);
                res += `Two distinct real roots:\\nx1 = ${x1.toFixed(4)}\\nx2 = ${x2.toFixed(4)}`;
            } else if (D === 0) {
                const x = -b / (2*a);
                res += `One repeated real root:\\nx = ${x.toFixed(4)}`;
            } else {
                const real = (-b / (2*a)).toFixed(4);
                const imag = (Math.sqrt(-D) / (2*a)).toFixed(4);
                res += `Complex roots:\\nx1 = ${real} + ${imag}i\\nx2 = ${real} - ${imag}i`;
            }
            out.innerText = res;
        }

        function factorial(n) {
            if (n === 0 || n === 1) return 1;
            for (var i = n - 1; i >= 1; i--) { n *= i; }
            return n;
        }

        function calcBinomial() {
            const n = parseInt(document.getElementById('math-n').value);
            const x = parseInt(document.getElementById('math-x').value);
            const p = parseFloat(document.getElementById('math-p').value);
            
            if(x > n || x < 0) { out.innerText = "Error: Invalid x for given n."; return; }
            if(p < 0 || p > 1) { out.innerText = "Error: Probability p must be between 0 and 1."; return; }
            
            const q = 1 - p;
            const comb = factorial(n) / (factorial(x) * factorial(n - x));
            const prob = comb * Math.pow(p, x) * Math.pow(q, n-x);
            
            out.innerText = `[MATHEMATICS] Binomial Probability calculated.\\nC(${n},${x}) = ${comb}\\nP(X = ${x}) = ${comb} × (${p}^${x}) × (${q}^${n-x})\\nResult = ${prob.toFixed(6)}`;
        }

        // Biology
        function calcHardyWeinberg() {
            const q = parseFloat(document.getElementById('bio-q').value);
            if(q < 0 || q > 1) { out.innerText = "Error: Frequency must be between 0 and 1."; return; }
            
            const p = 1 - q;
            const p2 = p * p;
            const two_pq = 2 * p * q;
            const q2 = q * q;
            
            out.innerText = `[BIOLOGY] Hardy-Weinberg Equilibrium calculated.\\np (Dominant allele freq) = ${p.toFixed(2)}\\nq (Recessive allele freq) = ${q.toFixed(2)}\\n\\nGenotype Frequencies:\\nHomozygous Dominant (p²) = ${p2.toFixed(4)}\\nHeterozygous (2pq) = ${two_pq.toFixed(4)}\\nHomozygous Recessive (q²) = ${q2.toFixed(4)}`;
        }

        function calcCardiacOutput() {
            const hr = parseFloat(document.getElementById('bio-hr').value);
            const sv = parseFloat(document.getElementById('bio-sv').value);
            
            const co_ml = hr * sv;
            const co_l = co_ml / 1000;
            
            out.innerText = `[BIOLOGY] Cardiac Output calculated.\\nCO = ${hr} bpm × ${sv} mL/beat\\nCO = ${co_ml} mL/min\\nCO = ${co_l.toFixed(2)} L/min`;
        }
    </script>
</body>
</html>
        """
        path = os.path.join(tempfile.gettempdir(), "sigma_ncert_calculator.html")
        with open(path, "w", encoding="utf-8") as f:
            f.write(html_content)
        webbrowser.open("file://" + os.path.realpath(path))
