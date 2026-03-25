from sigma_core.hal.native_libc import ApexLib
import tkinter as tk
from tkinter import ttk
# import os (Purged by Scrubber)
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED

class NcertCalcPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "NCERT MASTER CALCULATOR (CLASS 1-12)", "Physics, Chem, Math & Bio Omni-Solver")
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        card = self.controller._card(main_panel, "NCERT Omni-Calculator")
        card.master.pack(pady=50)
        
        tk.Label(card, text="The NCERT Master Calculator provides comprehensive pre-built formulas for Class 1 to 12.",
                 font=FONT_MED, bg=PAL["card"], fg=PAL["dim"], wraplength=400).pack(pady=20, padx=20)
        
        def _launch():
            self._generate_and_launch_html()
            self.controller._notify("NCERT Calc", "Class 1-12 Calculator launched.", "OK")

        ttk.Button(card, text="🧮 Launch NCERT Master Calculator", command=_launch, style="Teal.TButton").pack(pady=20)

    def _generate_and_launch_html(self):
        html_content = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SigmaOS NCERT Master Calculator (Class 1-12)</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: #0A0A12; color: #F2F2F7; margin: 0; padding: 20px; }
        .container { max-width: 1200px; margin: 0 auto; background-color: #11111E; padding: 20px; border-radius: 10px; box-shadow: 0 4px 15px rgba(0,0,0,0.5); }
        h1 { color: #5AC8FA; font-size: 26px; text-align: center; }
        .nav { display: flex; justify-content: center; gap: 10px; margin-bottom: 20px; flex-wrap: wrap; }
        .nav button { background-color: #1C1C1E; color: white; border: 1px solid #38383A; padding: 8px 15px; border-radius: 5px; cursor: pointer; font-weight: bold; margin-bottom: 5px;}
        .nav button.active { background-color: #5856D6; border-color: #5856D6; }
        .nav button:hover { background-color: #2C2C2E; }
        .nav button.active:hover { background-color: #AF52DE; }
        
        .tab-content { display: none; }
        .tab-content.active { display: block; }
        
        h2 { color: #FFCC00; font-size: 20px; border-bottom: 1px solid #38383A; padding-bottom: 10px; margin-top: 20px; }
        .grid-3 { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 15px; margin-top: 15px; }
        .card { background-color: #1C1C1E; padding: 15px; border-radius: 8px; border: 1px solid #38383A; }
        label { display: block; font-size: 13px; color: #8E8E93; margin-bottom: 4px; mt-2}
        input, select { width: 100%; box-sizing: border-box; padding: 8px; margin-bottom: 10px; background-color: #252529; color: white; border: 1px solid #5AC8FA; border-radius: 4px; font-family: monospace; }
        .btn { background-color: #5AC8FA; color: #0A0A12; padding: 10px; width: 100%; border: none; border-radius: 4px; cursor: pointer; font-weight: bold; font-size: 14px; transition: 0.2s; }
        .btn:hover { background-color: #4CD964; }
        
        /* Category specific button colors */
        .btn-math { background-color: #FF9F0A; color: white;}
        .btn-phys { background-color: #5AC8FA; color: black; }
        .btn-chem { background-color: #4CD964; color: black; }
        .btn-bio { background-color: #BF5AF2; color: white; }
        
        .output-box { background-color: #0A0A12; padding: 15px; border-radius: 4px; font-family: 'Consolas', monospace; color: #4CD964; margin-top: 15px; border: 1px solid #4CD964; white-space: pre-wrap; font-size: 15px; height: 150px; overflow-y: auto;}
        .formula { font-style: italic; color: #FF3B30; font-size: 12px; margin-top: -5px; margin-bottom: 10px;}
    </style>
</head>
<body>
    <div class="container">
        <h1>🧮 NCERT Master Omni-Calculator (Class 1-12)</h1>
        
        <div class="nav">
            <button class="active" onclick="switchTab('primary')">🎒 Primary (Class 1-5)</button>
            <button onclick="switchTab('middle')">📘 Middle (Class 6-8)</button>
            <button onclick="switchTab('secondary')">📗 Secondary (Class 9-10)</button>
            <button onclick="switchTab('senior')">🎓 Senior Sec (Class 11-12)</button>
        </div>
        
        <!-- PRIMARY (1-5) -->
        <div id="primary" class="tab-content active">
            <h2>🎒 Primary Math (Class 1-5)</h2>
            <div class="grid-3">
                <div class="card">
                    <h3 style="color: #FF9F0A; margin-top: 0;">Basic Arithmetic</h3>
                    <div class="formula">+, -, ×, ÷</div>
                    <label>Number A</label>
                    <input type="number" id="prim-a" value="10">
                    <label>Operation</label>
                    <select id="prim-op">
                        <option value="+">Addition (+)</option>
                        <option value="-">Subtraction (-)</option>
                        <option value="*">Multiplication (×)</option>
                        <option value="/">Division (÷)</option>
                    </select>
                    <label>Number B</label>
                    <input type="number" id="prim-b" value="5">
                    <button class="btn btn-math" onclick="calcArith()">Calculate</button>
                </div>
                
                <div class="card">
                    <h3 style="color: #FF9F0A; margin-top: 0;">Perimeter & Area (Rectangle/Square)</h3>
                    <div class="formula">P=2(l+w), A=l*w</div>
                    <label>Length (l)</label>
                    <input type="number" id="prim-l" value="10">
                    <label>Width (w) [Use same as length for Square]</label>
                    <input type="number" id="prim-w" value="5">
                    <button class="btn btn-math" onclick="calcRect()">Calculate P & A</button>
                </div>
                
                <div class="card">
                    <h3 style="color: #FF9F0A; margin-top: 0;">Fractions to Decimals</h3>
                    <div class="formula">Numerator / Denominator</div>
                    <label>Numerator</label>
                    <input type="number" id="prim-num" value="3">
                    <label>Denominator</label>
                    <input type="number" id="prim-den" value="4">
                    <button class="btn btn-math" onclick="calcFraction()">Convert to % & Decimal</button>
                </div>
            </div>
        </div>

        <!-- MIDDLE (6-8) -->
        <div id="middle" class="tab-content">
            <h2>📘 Middle School Data (Class 6-8)</h2>
            <div class="grid-3">
                <div class="card">
                    <h3 style="color: #FF9F0A; margin-top: 0;">Percentages</h3>
                    <div class="formula">Value / Total × 100</div>
                    <label>Value Obtained</label>
                    <input type="number" id="mid-val" value="40">
                    <label>Total Value</label>
                    <input type="number" id="mid-tot" value="50">
                    <button class="btn btn-math" onclick="calcPercent()">Find Percentage</button>
                </div>
                
                <div class="card">
                    <h3 style="color: #FF9F0A; margin-top: 0;">Simple Interest</h3>
                    <div class="formula">SI = (P × R × T) / 100</div>
                    <label>Principal (P)</label>
                    <input type="number" id="mid-p" value="1000">
                    <label>Rate % (R)</label>
                    <input type="number" id="mid-r" value="5">
                    <label>Time in years (T)</label>
                    <input type="number" id="mid-t" value="2">
                    <button class="btn btn-math" onclick="calcSI()">Calculate Interest</button>
                </div>
                
                <div class="card">
                    <h3 style="color: #5AC8FA; margin-top: 0;">Speed, Distance & Time</h3>
                    <div class="formula">S = D / T</div>
                    <label>Distance (m/km)</label>
                    <input type="number" id="mid-dist" value="100">
                    <label>Time (s/hr)</label>
                    <input type="number" id="mid-time" value="10">
                    <button class="btn btn-phys" onclick="calcSpeed()">Calculate Speed</button>
                </div>
                
                <div class="card">
                    <h3 style="color: #4CD964; margin-top: 0;">Density</h3>
                    <div class="formula">Density = Mass / Volume</div>
                    <label>Mass (g or kg)</label>
                    <input type="number" id="mid-mass" value="50">
                    <label>Volume (cm³ or L)</label>
                    <input type="number" id="mid-vol" value="10">
                    <button class="btn btn-chem" onclick="calcDensity()">Calculate Density</button>
                </div>
            </div>
        </div>

        <!-- SECONDARY (9-10) -->
        <div id="secondary" class="tab-content">
            <h2>📗 Secondary Sciences & Math (Class 9-10)</h2>
            <div class="grid-3">
                <div class="card">
                    <h3 style="color: #FF9F0A; margin-top: 0;">Pythagoras Theorem</h3>
                    <div class="formula">c² = a² + b²</div>
                    <label>Base (a)</label>
                    <input type="number" id="sec-a" value="3">
                    <label>Perpendicular (b)</label>
                    <input type="number" id="sec-b" value="4">
                    <button class="btn btn-math" onclick="calcPyth()">Find Hypotenuse (c)</button>
                </div>
                
                <div class="card">
                    <h3 style="color: #FF9F0A; margin-top: 0;">Compound Interest</h3>
                    <div class="formula">A = P(1 + R/100)^nt</div>
                    <label>Principal (P)</label>
                    <input type="number" id="sec-p" value="1000">
                    <label>Rate (R)</label>
                    <input type="number" id="sec-r" value="5">
                    <label>Time (T)</label>
                    <input type="number" id="sec-t" value="2">
                    <button class="btn btn-math" onclick="calcCI()">Calculate Amt</button>
                </div>
                
                <div class="card">
                    <h3 style="color: #5AC8FA; margin-top: 0;">Newton's 2nd Law</h3>
                    <div class="formula">F = ma</div>
                    <label>Mass (kg)</label>
                    <input type="number" id="sec-m" value="10">
                    <label>Acceleration (m/s²)</label>
                    <input type="number" id="sec-acc" value="9.8">
                    <button class="btn btn-phys" onclick="calcForce()">Calculate Force (N)</button>
                </div>
                
                <div class="card">
                    <h3 style="color: #5AC8FA; margin-top: 0;">Work, Energy & Power</h3>
                    <div class="formula">W = Fd, P = W/t, K.E.=½mv², P.E.=mgh</div>
                    <label>Mass (kg)</label>
                    <input type="number" id="sec-we-m" value="10">
                    <label>Velocity (m/s) or Height (m)</label>
                    <input type="number" id="sec-we-v" value="5">
                    <button class="btn btn-phys" onclick="calcEnergy()">Calculate KE/PE</button>
                </div>
                
                <div class="card">
                    <h3 style="color: #BF5AF2; margin-top: 0;">BMI Calculator</h3>
                    <div class="formula">BMI = Weight(kg) / Height(m)²</div>
                    <label>Weight (kg)</label>
                    <input type="number" id="sec-w" value="70">
                    <label>Height (meters)</label>
                    <input type="number" id="sec-h" value="1.75">
                    <button class="btn btn-bio" onclick="calcBMI()">Calculate BMI</button>
                </div>
                
                <div class="card">
                    <h3 style="color: #4CD964; margin-top: 0;">Mole Concept</h3>
                    <div class="formula">Moles = Given Mass / Molar Mass</div>
                    <label>Given Mass (g)</label>
                    <input type="number" id="sec-gm" value="18">
                    <label>Molar Mass (g/mol) [e.g., H2O=18]</label>
                    <input type="number" id="sec-mm" value="18">
                    <button class="btn btn-chem" onclick="calcMole()">Calculate Moles</button>
                </div>
            </div>
        </div>

        <!-- SENIOR (11-12) -->
        <div id="senior" class="tab-content">
            <h2>🎓 Senior Secondary Advanced (Class 11-12)</h2>
            <div class="grid-3">
                <!-- Physics: Kinematics -->
                <div class="card">
                    <h3 style="color: #5AC8FA; margin-top: 0;">Kinematics: Equations of Motion</h3>
                    <div class="formula">v = u+at | s = ut+½at² | v² = u²+2as</div>
                    <label>Initial Velocity (u) [m/s]</label>
                    <input type="number" id="sen-u" value="0">
                    <label>Acceleration (a) [m/s²]</label>
                    <input type="number" id="sen-a" value="9.8">
                    <label>Time (t) [s]</label>
                    <input type="number" id="sen-t" value="5">
                    <button class="btn btn-phys" onclick="calcSeniorKinematics()">Calculate v & s</button>
                </div>
                
                <!-- Physics: Optics -->
                <div class="card">
                    <h3 style="color: #5AC8FA; margin-top: 0;">Optics: Lens Maker’s Formula</h3>
                    <div class="formula">1/f = (μ-1)(1/R1 - 1/R2)</div>
                    <label>Refractive Index (μ)</label>
                    <input type="number" id="sen-mu" value="1.5">
                    <label>R1 (cm)</label>
                    <input type="number" id="sen-r1" value="10">
                    <label>R2 (cm)</label>
                    <input type="number" id="sen-r2" value="-10">
                    <button class="btn btn-phys" onclick="calcLens()">Calculate Focal Length</button>
                </div>

                <!-- Chemistry: Molarity -->
                <div class="card">
                    <h3 style="color: #4CD964; margin-top: 0;">Solutions: Molarity & pH</h3>
                    <div class="formula">M=n/V | pH = -log[H+]</div>
                    <label>[H+] Concentration (M)</label>
                    <input type="number" id="sen-h" value="0.001">
                    <button class="btn btn-chem" onclick="calcPh()">Calculate pH</button>
                </div>
                
                <!-- Chemistry: Ideal Gas -->
                <div class="card">
                    <h3 style="color: #4CD964; margin-top: 0;">States of Matter: Ideal Gas</h3>
                    <div class="formula">PV = nRT (R=0.0821)</div>
                    <label>Pressure (atm)</label>
                    <input type="number" id="sen-p" value="1">
                    <label>Moles (n)</label>
                    <input type="number" id="sen-n" value="2">
                    <label>Temperature (K)</label>
                    <input type="number" id="sen-temp" value="298">
                    <button class="btn btn-chem" onclick="calcGasLaw()">Calculate V</button>
                </div>

                <!-- Math: Quadratic -->
                <div class="card">
                    <h3 style="color: #FF9F0A; margin-top: 0;">Algebra: Quadratic Engine</h3>
                    <div class="formula">Complex Roots: ax² + bx + c = 0</div>
                    <label>a</label>
                    <input type="number" id="sen-qa" value="1">
                    <label>b</label>
                    <input type="number" id="sen-qb" value="-5">
                    <label>c</label>
                    <input type="number" id="sen-qc" value="6">
                    <button class="btn btn-math" onclick="calcQuadratic()">Solve Roots</button>
                </div>
                
                <!-- Math: Binomial -->
                <div class="card">
                    <h3 style="color: #FF9F0A; margin-top: 0;">Statistics: Binomial Dist</h3>
                    <div class="formula">P(x) = nCx * p^x * q^(n-x)</div>
                    <label>n (trials)</label>
                    <input type="number" id="sen-bn" value="10">
                    <label>x (successes)</label>
                    <input type="number" id="sen-bx" value="5">
                    <label>p (prob success)</label>
                    <input type="number" id="sen-bp" value="0.5">
                    <button class="btn btn-math" onclick="calcBinomial()">Calculate P(X)</button>
                </div>
                
                <!-- Bio: Hardy -->
                <div class="card">
                    <h3 style="color: #BF5AF2; margin-top: 0;">Genetics: Hardy-Weinberg Eq</h3>
                    <div class="formula">p² + 2pq + q² = 1</div>
                    <label>Freq of recessive allele (q)</label>
                    <input type="number" id="sen-bq" value="0.4" step="0.01" max="1" min="0">
                    <button class="btn btn-bio" onclick="calcHardyWeinberg()">Calc Frequencies</button>
                </div>
            </div>
        </div>

        <div class="card" style="margin-top: 20px;">
            <div style="display: flex; justify-content: space-between; align-items: center;">
                <h2 style="margin: 0; border: none; padding: 0;">Master Computation Yield</h2>
                <button onclick="document.getElementById('master-out').innerText='Cleared.'" style="background: transparent; color: #FF3B30; border: 1px solid #FF3B30; padding: 5px 10px; cursor:pointer;">Clear Log</button>
            </div>
            <div class="output-box" id="master-out">Awaiting input...</div>
        </div>
    </div>

    <script>
        const out = document.getElementById('master-out');
        
        function logMsg(msg) {
            out.innerText = msg + "\\n\\n---\\n" + out.innerText;
        }

        function switchTab(tabId) {
            document.querySelectorAll('.tab-content').forEach(el => el.classList.remove('active'));
            document.querySelectorAll('.nav button').forEach(el => el.classList.remove('active'));
            document.getElementById(tabId).classList.add('active');
            event.target.classList.add('active');
        }

        // Primaey Methods
        function calcArith() {
            const a = parseFloat(document.getElementById('prim-a').value);
            const b = parseFloat(document.getElementById('prim-b').value);
            const op = document.getElementById('prim-op').value;
            let res = 0;
            if(op === '+') res = a + b;
            if(op === '-') res = a - b;
            if(op === '*') res = a * b;
            if(op === '/') {
                if(b===0) { logMsg(`Error: Division by Zero`); return;}
                res = a / b;
            }
            logMsg(`[MATH: Primary] ${a} ${op} ${b} = ${res}`);
        }
        function calcRect() {
            const l = parseFloat(document.getElementById('prim-l').value);
            const w = parseFloat(document.getElementById('prim-w').value);
            logMsg(`[MATH: Geometry] Perimeter = 2*(${l}+${w}) = ${2*(l+w)}\\nArea = ${l} * ${w} = ${l*w}`);
        }
        function calcFraction() {
            const n = parseFloat(document.getElementById('prim-num').value);
            const d = parseFloat(document.getElementById('prim-den').value);
            if(d===0) {logMsg('Error: Zero Denominator'); return;}
            logMsg(`[MATH: Fractions] ${n}/${d} = ${(n/d).toFixed(4)} = ${((n/d)*100).toFixed(2)}%`);
        }

        // Middle Methods
        function calcPercent() {
            const v = parseFloat(document.getElementById('mid-val').value);
            const t = parseFloat(document.getElementById('mid-tot').value);
            logMsg(`[MATH: Middle] Percentage = (${v} / ${t}) * 100 = ${((v/t)*100).toFixed(2)}%`);
        }
        function calcSI() {
            const p = parseFloat(document.getElementById('mid-p').value);
            const r = parseFloat(document.getElementById('mid-r').value);
            const t = parseFloat(document.getElementById('mid-t').value);
            const si = (p*r*t)/100;
            logMsg(`[MATH: Finance] Simple Interest = (${p} × ${r} × ${t}) / 100 = ${si}\\nTotal Amount = ${p+si}`);
        }
        function calcSpeed() {
            const d = parseFloat(document.getElementById('mid-dist').value);
            const t = parseFloat(document.getElementById('mid-time').value);
            if(t===0) return;
            logMsg(`[PHYS: Kinematics] Speed = ${d} / ${t} = ${(d/t).toFixed(2)} units`);
        }
        function calcDensity() {
            const m = parseFloat(document.getElementById('mid-mass').value);
            const v = parseFloat(document.getElementById('mid-vol').value);
            if(v===0) return;
            logMsg(`[CHEM: Matter] Density = Mass / Vol = ${m} / ${v} = ${(m/v).toFixed(4)}`);
        }

        // Secondary Methods
        function calcPyth() {
            const a = parseFloat(document.getElementById('sec-a').value);
            const b = parseFloat(document.getElementById('sec-b').value);
            const c = Math.sqrt(a*a + b*b);
            logMsg(`[MATH: Geometry] c² = a² + b²\\nHypotenuse = √(${a*a} + ${b*b}) = ${c.toFixed(4)}`);
        }
        function calcCI() {
            const p = parseFloat(document.getElementById('sec-p').value);
            const r = parseFloat(document.getElementById('sec-r').value);
            const t = parseFloat(document.getElementById('sec-t').value);
            const a = p * Math.pow((1 + r/100), t);
            logMsg(`[MATH: Finance] Compound Interest Amount = ${p} * (1 + ${r}/100)^${t} = ${a.toFixed(2)}`);
        }
        function calcForce() {
            const m = parseFloat(document.getElementById('sec-m').value);
            const a = parseFloat(document.getElementById('sec-acc').value);
            logMsg(`[PHYS: Dynamics] Force = m*a\\nF = ${m} × ${a} = ${(m*a).toFixed(2)} N`);
        }
        function calcEnergy() {
            const m = parseFloat(document.getElementById('sec-we-m').value);
            const v = parseFloat(document.getElementById('sec-we-v').value); // Using v as height for PE
            const ke = 0.5 * m * v * v;
            const pe = m * 9.8 * v;
            logMsg(`[PHYS: Energy] If v is velocity: K.E. = ½*${m}*${v}² = ${ke.toFixed(2)} Joules\\nIf v is height: P.E. = ${m}*9.8*${v} = ${pe.toFixed(2)} Joules`);
        }
        function calcBMI() {
            const w = parseFloat(document.getElementById('sec-w').value);
            const h = parseFloat(document.getElementById('sec-h').value);
            const bmi = w / (h*h);
            logMsg(`[BIO: Health] BMI = ${w} / (${h}²) = ${bmi.toFixed(2)} kg/m²`);
        }
        function calcMole() {
            const gm = parseFloat(document.getElementById('sec-gm').value);
            const mm = parseFloat(document.getElementById('sec-mm').value);
            logMsg(`[CHEM: Moles] Moles = ${gm} / ${mm} = ${(gm/mm).toFixed(4)} mol`);
        }

        // Senior Methods
        function calcSeniorKinematics() {
            const u = parseFloat(document.getElementById('sen-u').value);
            const a = parseFloat(document.getElementById('sen-a').value);
            const t = parseFloat(document.getElementById('sen-t').value);
            const v = u + (a * t);
            const s = (u*t) + (0.5 * a * t * t);
            logMsg(`[PHYSICS] 3 Eq of Motion evaluated.\\nv = ${u} + (${a}×${t}) = ${v} m/s\\ns = (${u}×${t}) + ½(${a}×${t}²) = ${s} m\\nv² = u² + 2as => ${v*v} = ${u*u} + ${2*a*s}`);
        }
        
        function calcLens() {
            const mu = parseFloat(document.getElementById('sen-mu').value);
            const r1 = parseFloat(document.getElementById('sen-r1').value);
            const r2 = parseFloat(document.getElementById('sen-r2').value);
            if(r1===0 || r2===0) { logMsg("Error"); return;}
            const invF = (mu - 1) * ((1/r1) - (1/r2));
            const f = 1/invF;
            logMsg(`[PHYSICS] Lens Maker's Formula.\\nf = 1 / [(${mu}-1)*(1/${r1} - 1/${r2})]\\nf = ${f.toFixed(4)} cm`);
        }
        
        function calcPh() {
            const h = parseFloat(document.getElementById('sen-h').value);
            if(h<=0) return;
            const ph = -Math.log10(h);
            logMsg(`[CHEMISTRY] pH = -log10(${h}) = ${ph.toFixed(4)}`);
        }

        function calcGasLaw() {
            const p = parseFloat(document.getElementById('sen-p').value);
            const n = parseFloat(document.getElementById('sen-n').value);
            const t = parseFloat(document.getElementById('sen-temp').value);
            const r = 0.0821; 
            if(p===0) return;
            const v = (n * r * t) / p;
            logMsg(`[CHEMISTRY] Ideal Gas Volume calculated.\\nV = (${n} × ${r} × ${t}) / ${p} = ${v.toFixed(3)} L`);
        }

        function calcQuadratic() {
            const a = parseFloat(document.getElementById('sen-qa').value);
            const b = parseFloat(document.getElementById('sen-qb').value);
            const c = parseFloat(document.getElementById('sen-qc').value);
            const D = (b*b) - (4*a*c);
            let res = `[MATHEMATICS] Discriminant (D) = ${D}\\n`;
            if(D > 0) {
                res += `x1 = ${((-b + Math.sqrt(D)) / (2*a)).toFixed(4)}\nx2 = ${((-b - Math.sqrt(D)) / (2*a)).toFixed(4)}`;
            } else if (D === 0) {
                res += `Repeated Root x = ${(-b / (2*a)).toFixed(4)}`;
            } else {
                const real = (-b / (2*a)).toFixed(4);
                const imag = (Math.sqrt(-D) / (2*a)).toFixed(4);
                res += `Complex roots: ${real} ± ${imag}i`;
            }
            logMsg(res);
        }

        function factorial(n) {
            if (n === 0 || n === 1) return 1;
            for (var i = n - 1; i >= 1; i--) { n *= i; }
            return n;
        }

        function calcBinomial() {
            const n = parseInt(document.getElementById('sen-bn').value);
            const x = parseInt(document.getElementById('sen-bx').value);
            const p = parseFloat(document.getElementById('sen-bp').value);
            const q = 1 - p;
            const comb = factorial(n) / (factorial(x) * factorial(n - x));
            const prob = comb * Math.pow(p, x) * Math.pow(q, n-x);
            logMsg(`[MATHEMATICS] P(X=${x}) = C(${n},${x}) × (${p}^${x}) × (${q}^${n-x}) = ${prob.toFixed(6)}`);
        }

        function calcHardyWeinberg() {
            const q = parseFloat(document.getElementById('sen-bq').value);
            const p = 1 - q;
            logMsg(`[BIOLOGY] Hardy-Weinberg Equilibrium\\np = ${p.toFixed(2)}, q = ${q.toFixed(2)}\\nHomozygous Dominant (p²) = ${(p*p).toFixed(4)}\\nHeterozygous (2pq) = ${(2*p*q).toFixed(4)}\\nHomozygous Recessive (q²) = ${(q*q).toFixed(4)}`);
        }
    </script>
</body>
</html>
        """
        path = os.path.join(tempfile.gettempdir(), "sigma_ncert_calculator.html")
        with open(path, "w", encoding="utf-8") as f:
            f.write(html_content)
        webbrowser.open("file://" + os.path.realpath(path))
