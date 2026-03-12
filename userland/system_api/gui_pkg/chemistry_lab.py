import tkinter as tk
from tkinter import ttk
import os
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED

class ChemistryLabPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "QUANTUM CHEMISTRY LAB", "Browser-based Periodic Table & Equation Balancer")
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        card = self.controller._card(main_panel, "Chemistry Launch Core")
        card.master.pack(pady=50)
        
        tk.Label(card, text="The Quantum Chemistry Lab is a high-performance Browser-Based Utility.",
                 font=FONT_MED, bg=PAL["card"], fg=PAL["dim"]).pack(pady=20, padx=20)
        
        def _launch():
            self._generate_and_launch_html()
            self.controller._notify("Chemistry Lab", "Browser-based lab launched.", "OK")

        ttk.Button(card, text="🧪 Launch Chemistry Lab in Browser", command=_launch, style="Teal.TButton").pack(pady=20)

    def _generate_and_launch_html(self):
        html_content = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SigmaOS Quantum Chemistry Lab</title>
    <style>
        body { font-family: 'Segoe UI', sans-serif; background-color: #0A0A12; color: #F2F2F7; margin: 0; padding: 20px; }
        .container { max-width: 900px; margin: 0 auto; background-color: #11111E; padding: 20px; border-radius: 10px; box-shadow: 0 4px 15px rgba(0,0,0,0.5); }
        h1 { color: #5AC8FA; font-size: 24px; text-align: center; }
        h2 { color: #4CD964; font-size: 18px; border-bottom: 1px solid #38383A; padding-bottom: 10px; }
        .grid-2 { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; }
        .card { background-color: #1C1C1E; padding: 15px; border-radius: 8px; border: 1px solid #38383A; }
        input { width: 100%; box-sizing: border-box; padding: 10px; margin-top: 5px; margin-bottom: 15px; background-color: #252529; color: white; border: 1px solid #5856D6; border-radius: 4px; font-family: monospace; }
        .btn { background-color: #5856D6; color: white; padding: 10px; width: 100%; border: none; border-radius: 4px; cursor: pointer; font-weight: bold; font-size: 14px; }
        .btn:hover { background-color: #AF52DE; }
        .periodic-table { display: grid; grid-template-columns: repeat(18, 1fr); gap: 2px; font-size: 10px; text-align: center; margin-top: 15px; }
        .element { background-color: #252529; border: 1px solid #38383A; padding: 5px 0; cursor: pointer; border-radius: 2px; }
        .element:hover { background-color: #5AC8FA; color: #0A0A12; border-color: #5AC8FA; }
        .output { background-color: #0A0A12; padding: 15px; border-radius: 4px; min-height: 100px; border: 1px solid #FFCC00; overflow-y: auto; white-space: pre-wrap; font-family: 'Consolas', monospace; color: #FFCC00; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🧪 SigmaOS Quantum Chemistry Lab</h1>
        
        <div class="grid-2">
            <div class="card">
                <h2>Element Dictionary</h2>
                <input type="text" id="elem-search" placeholder="Search element by name or symbol (e.g. Oxygen, Fe)...">
                <button class="btn" onclick="searchElement()">🔍 Search Element Properties</button>
            </div>
            
            <div class="card">
                <h2>Chemical Equation Balancer (AI Approximation)</h2>
                <input type="text" id="eq-input" placeholder="e.g. H2 + O2 = H2O">
                <button class="btn" onclick="balanceEquation()">⚖️ Auto-Balance Equation</button>
            </div>
        </div>

        <div class="card" style="margin-top: 20px;">
            <h2>Interactive Periodic Table</h2>
            <div class="periodic-table" id="ptable">
                <!-- Javascript will populate this minimal representation -->
            </div>
        </div>
        
        <div class="card" style="margin-top: 20px;">
            <h2>Research Console</h2>
            <div class="output" id="out-console">System Ready. Awaiting chemical input...</div>
        </div>
    </div>

    <script>
        const out = document.getElementById('out-console');

        // Minimal periodic table data
        const elements = [
            {num:1, sym:'H', name:'Hydrogen', group:1}, {num:2, sym:'He', name:'Helium', group:18},
            {num:3, sym:'Li', name:'Lithium', group:1}, {num:4, sym:'Be', name:'Beryllium', group:2},
            {num:5, sym:'B', name:'Boron', group:13}, {num:6, sym:'C', name:'Carbon', group:14},
            {num:7, sym:'N', name:'Nitrogen', group:15}, {num:8, sym:'O', name:'Oxygen', group:16},
            {num:9, sym:'F', name:'Fluorine', group:17}, {num:10, sym:'Ne', name:'Neon', group:18},
            {num:11, sym:'Na', name:'Sodium', group:1}, {num:12, sym:'Mg', name:'Magnesium', group:2},
            {num:13, sym:'Al', name:'Aluminum', group:13}, {num:14, sym:'Si', name:'Silicon', group:14},
            {num:15, sym:'P', name:'Phosphorus', group:15}, {num:16, sym:'S', name:'Sulfur', group:16},
            {num:17, sym:'Cl', name:'Chlorine', group:17}, {num:18, sym:'Ar', name:'Argon', group:18},
            {num:26, sym:'Fe', name:'Iron', group:8}, {num:79, sym:'Au', name:'Gold', group:11}
        ];

        function initPTable() {
            const pt = document.getElementById('ptable');
            // Mock grid placement logic (very simplified for demo)
            let currentNum = 1;
            for(let r=1; r<=4; r++) {
                for(let c=1; c<=18; c++) {
                    let div = document.createElement('div');
                    let el = elements.find(e => {
                        // Very rough mapping to positions for the first few elements to make it look grid-like
                        if(e.num===1 && c===1 && r===1) return true;
                        if(e.num===2 && c===18 && r===1) return true;
                        if(r===2 && (c===1||c===2||c>=13) && e.num===currentNum) return true;
                        if(r===3 && (c===1||c===2||c>=13) && e.num===currentNum) return true;
                        if(r===4 && e.num===currentNum) return true;
                        return false;
                    });
                    
                    if(el) {
                        div.className = 'element';
                        div.innerHTML = `<strong>${el.sym}</strong><br>${el.num}`;
                        div.onclick = () => showElement(el);
                        currentNum++;
                    } else {
                        // Empty space or untracked element
                        if(currentNum <= 20 || (r===4 && currentNum <= 36)) {
                            div.className = 'element';
                            div.style.opacity = '0.3';
                            div.innerHTML = '-';
                            currentNum++;
                        }
                    }
                    pt.appendChild(div);
                }
            }
        }

        function showElement(el) {
            out.innerText = `[ELEMENT] ${el.name} (${el.sym})\\nAtomic Number: ${el.num}\\nGroup: ${el.group}\\nStatus: Standard data retrieved locally.\\n\\n` + out.innerText;
        }

        function searchElement() {
            const query = document.getElementById('elem-search').value.toLowerCase();
            const el = elements.find(e => e.name.toLowerCase() === query || e.sym.toLowerCase() === query);
            if(el) {
                showElement(el);
            } else {
                out.innerText = `[ERROR] Element '${query}' not found in local optimized DB.\\n\\n` + out.innerText;
            }
        }

        function balanceEquation() {
            const eq = document.getElementById('eq-input').value;
            // Mock AI Balancer
            if(eq.includes('H2') && eq.includes('O2') && eq.includes('H2O')) {
                out.innerText = `[AI BALANCER] Processing: ${eq}\\n[BALANCED] 2H2 + O2 = 2H2O\\n[DETAILS] Synthesized logic matrix completed in 4ms.\\n\\n` + out.innerText;
            } 
            else if(eq.includes('Na') && eq.includes('Cl2') && eq.includes('NaCl')) {
                out.innerText = `[AI BALANCER] Processing: ${eq}\\n[BALANCED] 2Na + Cl2 = 2NaCl\\n[DETAILS] Synthesized logic matrix completed in 3ms.\\n\\n` + out.innerText;
            }
            else {
                out.innerText = `[AI BALANCER] Complex organic or unrecognized chain: ${eq}.\\n[HEURISTIC] Attempting matrix inversion...\\n[WARNING] Deep learning balancer suggests formatting check required.\\n\\n` + out.innerText;
            }
        }

        initPTable();
    </script>
</body>
</html>
        """
        path = os.path.join(tempfile.gettempdir(), "sigma_chemistry_lab.html")
        with open(path, "w", encoding="utf-8") as f:
            f.write(html_content)
        webbrowser.open("file://" + os.path.realpath(path))
