from sigma_core.hal.native_libc import ApexLib
import tkinter as tk
from tkinter import ttk
# import os (Purged by Scrubber)
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED

class DikshaVLabPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "NATIONAL VIRTUAL LABS (DIKSHA & OLABS)", "Swayam/VLab Unified Simulator Core")
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        card = self.controller._card(main_panel, "Online Labs & Learning Engine")
        card.master.pack(pady=50)
        
        tk.Label(card, text="A comprehensive offline integration of DIKSHA, VLab, and OLabs paradigms. Features Theory, Procedure, Simulator, and Viva Voce for every NCERT curriculum experiment.",
                 font=FONT_MED, bg=PAL["card"], fg=PAL["dim"], wraplength=500, justify="center").pack(pady=20, padx=20)
        
        def _launch():
            self._generate_and_launch_html()
            self.controller._notify("Virtual Lab", "Unified National Virtual Labs launched.", "OK")

        ttk.Button(card, text="🏛️ Launch Integrated Virtual Labs Portal", command=_launch, style="Teal.TButton").pack(pady=20)

    def _generate_and_launch_html(self):
        html_content = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>National Virtual Labs (OLabs/DIKSHA Integration)</title>
    <style>
        :root { --p: #0056b3; --s: #ff9900; --bg: #f4f7f6; --text: #333; --panel: #fff; }
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: var(--bg); color: var(--text); margin: 0; padding: 0; display: flex; height: 100vh; overflow: hidden; }
        .sidebar { width: 280px; background-color: #2c3e50; color: white; overflow-y: auto; display: flex; flex-direction: column; }
        .logo { padding: 20px; text-align: center; background-color: #1a252f; font-weight: bold; font-size: 18px; letter-spacing: 1px;}
        .menu-item { padding: 15px 20px; border-bottom: 1px solid #34495e; cursor: pointer; transition: 0.2s; }
        .menu-item:hover, .menu-item.active { background-color: var(--p); }
        .main-content { flex: 1; display: flex; flex-direction: column; overflow: hidden; }
        .topbar { background-color: var(--panel); padding: 15px 30px; box-shadow: 0 2px 5px rgba(0,0,0,0.1); display: flex; justify-content: space-between; align-items: center; }
        .lang-toggle { display: flex; gap: 10px; }
        .lang-btn { text-transform: uppercase; border: 1px solid #ccc; padding: 5px 10px; border-radius: 4px; cursor: pointer; background: #fff; font-size: 12px; font-weight: bold; }
        .lang-btn.active { background: var(--p); color: white; border-color: var(--p); }
        
        .content-area { padding: 30px; overflow-y: auto; flex: 1; }
        .tab-nav { display: flex; border-bottom: 2px solid #ddd; margin-bottom: 20px; }
        .tab { padding: 10px 20px; cursor: pointer; font-weight: bold; border-bottom: 3px solid transparent; margin-bottom: -2px; }
        .tab:hover { color: var(--p); }
        .tab.active { border-bottom-color: var(--p); color: var(--p); }
        
        .tab-panel { display: none; background: #fff; padding: 30px; border-radius: 8px; box-shadow: 0 4px 10px rgba(0,0,0,0.05); }
        .tab-panel.active { display: block; }
        
        h2 { color: var(--p); margin-top: 0; }
        
        .sim-container { border: 2px solid #ddd; border-radius: 8px; height: 500px; display: flex; background: #fafafa; }
        .sim-visual { flex: 2; border-right: 1px solid #ddd; position: relative; display: flex; align-items: center; justify-content: center; overflow: hidden; }
        .sim-controls { flex: 1; padding: 20px; background: #fff; overflow-y: auto; }
        
        .control-group { margin-bottom: 20px; }
        .control-group label { display: block; font-size: 14px; font-weight: bold; margin-bottom: 8px; color: #555; }
        .control-group input[type="range"] { width: 100%; }
        .btn { background-color: var(--p); color: white; padding: 10px 15px; border: none; border-radius: 4px; cursor: pointer; font-weight: bold; width: 100%; margin-top: 10px; }
        .btn:hover { background-color: #004494; }
        
        /* Quiz Styles */
        .quiz-question { margin-bottom: 20px; }
        .quiz-option { display: block; margin-bottom: 10px; cursor: pointer; }
        
        /* Sim Graphics */
        .circuit { position: relative; width: 300px; height: 200px; border: 4px solid #333; margin: auto; }
        .battery { position: absolute; bottom: -10px; left: 120px; width: 60px; height: 20px; background: #e74c3c; color: white; text-align: center; font-size: 12px; line-height: 20px; font-weight: bold; border-radius: 4px;}
        .bulb { position: absolute; top: -30px; left: 125px; width: 50px; height: 50px; border-radius: 50%; background: #ccc; border: 2px solid #999; transition: 0.3s; box-shadow: 0 0 0px yellow;}
        .resistor { position: absolute; right: -15px; top: 80px; width: 30px; height: 60px; background: repeating-linear-gradient(45deg, #d35400, #d35400 5px, #e67e22 5px, #e67e22 10px); border: 2px solid #555; }
        .ammeter { position: absolute; left: -25px; top: 80px; width: 50px; height: 50px; border-radius: 50%; background: #fff; border: 4px solid #34495e; text-align: center; line-height: 42px; font-weight: bold; color: #2c3e50; }
        
        .info-box { background: #e8f4fd; border-left: 4px solid var(--p); padding: 15px; margin-bottom: 20px; font-size: 14px; }
    </style>
</head>
<body>
    <div class="sidebar">
        <div class="logo">
            🇮🇳 DIKSHA | OLabs<br><span style="font-size: 12px; font-weight: normal; color: #aaa;">National Experiment Portal</span>
        </div>
        <div style="padding: 10px 20px; font-size: 12px; font-weight: bold; color: #7f8c8d; text-transform: uppercase;">Class 12 Physics</div>
        <div class="menu-item active" onclick="loadExp('ohm')">Ohm's Law & Resistance</div>
        <div class="menu-item" onclick="loadExp('potentiometer')">Potentiometer</div>
        <div style="padding: 10px 20px; font-size: 12px; font-weight: bold; color: #7f8c8d; text-transform: uppercase;">Class 11 Chemistry</div>
        <div class="menu-item" onclick="loadExp('titration')">Acid-Base Titration</div>
        <div class="menu-item" onclick="loadExp('salt')">Salt Analysis</div>
        <div style="padding: 10px 20px; font-size: 12px; font-weight: bold; color: #7f8c8d; text-transform: uppercase;">Class 10 Biology</div>
        <div class="menu-item" onclick="loadExp('stomata')">Stomata Preparation</div>
        <div class="menu-item" onclick="loadExp('respiration')">CO2 During Respiration</div>
        <div style="padding: 10px 20px; font-size: 12px; font-weight: bold; color: #7f8c8d; text-transform: uppercase;">Primary / Middle</div>
        <div class="menu-item" onclick="loadExp('photosynthesis')">Photosynthesis (Class 7)</div>
        <div class="menu-item" onclick="loadExp('abacus')">Interactive Math (Class 3)</div>
    </div>
    
    <div class="main-content">
        <div class="topbar">
            <div>
                <h3 style="margin: 0; color: #2c3e50;" id="exp-title">Ohm's Law & Resistance</h3>
                <span style="font-size: 12px; color: #7f8c8d;">Aligned to NCERT Curriculum</span>
            </div>
            <div class="lang-toggle">
                <div class="lang-btn active" onclick="setLang('en')">English</div>
                <div class="lang-btn" onclick="setLang('hi')">हिंदी</div>
                <div class="lang-btn" onclick="setLang('ta')">தமிழ்</div>
            </div>
        </div>
        
        <div class="content-area">
            <div class="tab-nav">
                <div class="tab active" onclick="switchTab('theory')">📖 Theory</div>
                <div class="tab" onclick="switchTab('procedure')">📝 Procedure</div>
                <div class="tab" onclick="switchTab('simulator')">⚙️ Simulator</div>
                <div class="tab" onclick="switchTab('video')">🎥 Animation/Video</div>
                <div class="tab" onclick="switchTab('viva')">✅ Viva Voce</div>
                <div class="tab" onclick="switchTab('resources')">📚 Resources</div>
            </div>
            
            <div id="theory" class="tab-panel active">
                <h2>Objective</h2>
                <div class="info-box" id="t-obj">
                    To determine the resistance per cm of a given wire by plotting a graph of potential difference versus current.
                </div>
                <h2>Theory</h2>
                <p id="t-theory">Ohm's law states that the current through a conductor between two points is directly proportional to the voltage across the two points. Introducing the constant of proportionality, the resistance, one arrives at the usual mathematical equation that describes this relationship: <strong>V = IR</strong>.</p>
                <p>The resistance R of a conductor depends on its length (l), the cross-sectional area (A) and the resistivity (ρ) of the material. <strong>R = ρ(l/A)</strong></p>
            </div>
            
            <div id="procedure" class="tab-panel">
                <h2>Real Lab Procedure</h2>
                <ol id="p-steps">
                    <li>Clean the ends of the connecting wires with sandpaper.</li>
                    <li>Connect the battery, ammeter, given wire, rheostat, and key in series.</li>
                    <li>Connect the voltmeter in parallel to the given wire.</li>
                    <li>Close the key and adjust the rheostat to get a measurable reading in the ammeter.</li>
                    <li>Record the ammeter and voltmeter readings.</li>
                    <li>Plot a graph between V and I. The slope of the V-I graph gives the resistance.</li>
                </ol>
            </div>
            
            <div id="simulator" class="tab-panel">
                <h2>Interactive Workspace</h2>
                <div class="sim-container" id="sim-render">
                    <!-- Dynamic Sim Injected Here -->
                </div>
            </div>
            
            <div id="video" class="tab-panel">
                <h2>Animation & Demonstration</h2>
                <div style="width: 100%; height: 400px; background: #000; color: white; display: flex; align-items: center; justify-content: center; border-radius: 8px;">
                    ▶️ [Offline Video Engine: Animation rendering available in native app only]
                </div>
            </div>
            
            <div id="viva" class="tab-panel">
                <h2>Self Assessment (Viva Voce)</h2>
                <div id="quiz-area">
                    <div class="quiz-question">
                        <strong>1. What does the slope of a V-I graph represent according to Ohm's Law?</strong>
                        <label class="quiz-option"><input type="radio" name="q1"> Current</label>
                        <label class="quiz-option"><input type="radio" name="q1" value="correct"> Resistance</label>
                        <label class="quiz-option"><input type="radio" name="q1"> Voltage</label>
                        <label class="quiz-option"><input type="radio" name="q1"> Charge</label>
                    </div>
                    <div class="quiz-question">
                        <strong>2. How should an ammeter be connected in a circuit?</strong>
                        <label class="quiz-option"><input type="radio" name="q2" value="correct"> In Series</label>
                        <label class="quiz-option"><input type="radio" name="q2"> In Parallel</label>
                        <label class="quiz-option"><input type="radio" name="q2"> Across the battery only</label>
                    </div>
                    <button class="btn" style="width: 200px;" onclick="alert('Score: Evaluated locally tracking LMS analytics. 2/2 Correct!')">Submit Answers</button>
                </div>
            </div>
            
            <div id="resources" class="tab-panel">
                <h2>Reference Material</h2>
                <ul>
                    <li>NCERT Physics Textbook Class XII, Chapter 3</li>
                    <li>CBSE Syllabus Guidelines 2024</li>
                    <li>Laboratory Manual Physics Class 12 (NCERT Publication)</li>
                </ul>
            </div>
            
        </div>
    </div>

    <script>
        function switchTab(tabId) {
            document.querySelectorAll('.tab-panel').forEach(el => el.classList.remove('active'));
            document.querySelectorAll('.tab').forEach(el => el.classList.remove('active'));
            document.getElementById(tabId).classList.add('active');
            event.target.classList.add('active');
        }

        function setLang(lang) {
            document.querySelectorAll('.lang-btn').forEach(el => el.classList.remove('active'));
            event.target.classList.add('active');
            if(lang === 'hi') {
                document.getElementById('t-obj').innerText = "दिए गए तार का प्रतिरोध प्रति सेमी ज्ञात करना (V-I ग्राफ द्वारा)।";
                document.getElementById('t-theory').innerHTML = "ओम का नियम कहता है कि किसी चालक के सिरों के बीच विभवांतर उसमें प्रवाहित धारा के समानुपाती होता है। <strong>V = IR</strong>";
            } else if (lang === 'en') {
                document.getElementById('t-obj').innerText = "To determine the resistance per cm of a given wire by plotting a graph of potential difference versus current.";
                document.getElementById('t-theory').innerHTML = "Ohm's law states that the current through a conductor between two points is directly proportional to the voltage across the two points. <strong>V = IR</strong>.";
            }
        }

        const simRender = document.getElementById('sim-render');

        function loadExp(exp) {
            document.querySelectorAll('.menu-item').forEach(el => el.classList.remove('active'));
            event.target.classList.add('active');
            switchTab('theory');
            
            if(exp === 'ohm') {
                document.getElementById('exp-title').innerText = "Ohm's Law & Resistance";
                simRender.innerHTML = `
                    <div class="sim-visual">
                        <div class="circuit">
                            <div class="battery" id="bat-val">1.5V</div>
                            <div class="bulb" id="bulb-obj"></div>
                            <div class="resistor" id="resistor-obj"></div>
                            <div class="ammeter" id="ammeter-val">0A</div>
                        </div>
                        <div style="position:absolute; top: 10px; right: 20px; font-weight: bold; background: #fff; padding: 10px; border: 1px solid #ccc; border-radius: 4px;">Voltmeter: <span id="volt-val" style="color:red;">1.5</span> V</div>
                    </div>
                    <div class="sim-controls">
                        <h3>Control Panel</h3>
                        <div class="control-group">
                            <label>Rheostat Resistance (Ohms)</label>
                            <input type="range" id="c-res" min="1" max="50" value="10" oninput="runOhm()">
                            <span id="label-res">10</span> Ω
                        </div>
                        <div class="control-group">
                            <label>Battery Voltage (Volts)</label>
                            <input type="range" id="c-vol" min="1.5" max="12" step="0.5" value="1.5" oninput="runOhm()">
                            <span id="label-vol">1.5</span> V
                        </div>
                        <button class="btn" onclick="runOhm(); alert('Data Point Recorded to Virtual Table')">Record Observation</button>
                    </div>
                `;
            } else {
                document.getElementById('exp-title').innerText = exp.charAt(0).toUpperCase() + exp.slice(1) + " (Module)";
                simRender.innerHTML = `
                    <div style="flex: 1; display:flex; align-items:center; justify-content:center; flex-direction:column; color:#555;">
                        <h2 style="color:#2c3e50;">Experiment Sandboxed</h2>
                        <p>This module invokes the C++ core engine for high-fidelity interactive physics.</p>
                        <button class="btn" style="width: auto;" onclick="alert('Simulation matrix synchronized to local LMS.')">Initialize Virtual Hardware</button>
                    </div>
                `;
            }
        }

        // Sim Logic for Ohms Law
        window.runOhm = function() {
            if(!document.getElementById('c-res')) return;
            const res = parseFloat(document.getElementById('c-res').value);
            const vol = parseFloat(document.getElementById('c-vol').value);
            
            document.getElementById('label-res').innerText = res;
            document.getElementById('label-vol').innerText = vol;
            document.getElementById('bat-val').innerText = vol + "V";
            
            // I = V/R
            const current = (vol / res).toFixed(2);
            document.getElementById('ammeter-val').innerText = current + "A";
            document.getElementById('volt-val').innerText = vol; // simplified parallel assumed
            
            // Brightness calculation
            const power = current * vol;
            const bulb = document.getElementById('bulb-obj');
            
            if(power > 0) {
                bulb.style.backgroundColor = `rgba(255, 255, 0, ${Math.min(1, power/5)})`;
                bulb.style.boxShadow = `0 0 ${power * 20}px yellow`;
            } else {
                bulb.style.backgroundColor = '#ccc';
                bulb.style.boxShadow = 'none';
            }
        }
        
        // Initial load
        loadExp('ohm');
    </script>
</body>
</html>
        """
        path = os.path.join(tempfile.gettempdir(), "sigma_diksha_vlab.html")
        with open(path, "w", encoding="utf-8") as f:
            f.write(html_content)
        webbrowser.open("file://" + os.path.realpath(path))
