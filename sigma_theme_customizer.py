import json
import os
import sys

# --- SIGMA-OS THEME ENGINE & CUSTOMIZER v4.9 ---
# Allows absolute UI/UX personalization while enforcing Cyberpunk Resource Efficiency.

ROOT = os.getcwd()
THEME_DIR = os.path.join(ROOT, "userland", "apps", "theme_engine", "themes")

default_theme = {
    "sigma-bg": "#030303",
    "sigma-surface": "rgba(10, 15, 20, 0.4)",
    "sigma-border": "#333333",
    "sigma-accent-primary": "#00FFD2",
    "sigma-accent-secondary": "#8A2BE2",
    "sigma-text-main": "#E0E0E0",
    "sigma-text-muted": "#606060",
    "sigma-font-mono": "'JetBrains Mono', monospace",
    "sigma-glass-blur": "4px",
    "sigma-border-radius": "0px"
}

def ensure_theme_dir():
    if not os.path.exists(THEME_DIR):
        os.makedirs(THEME_DIR)

def save_theme(name: str, config: dict):
    ensure_theme_dir()
    path = os.path.join(THEME_DIR, f"{name}.json")
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(config, f, indent=4)
    print(f"✅ Theme '{name}' persisted to {path}")

def generate_css_bundle(theme_name: str):
    path = os.path.join(THEME_DIR, f"{theme_name}.json")
    if not os.path.exists(path):
        print(f"❌ Theme {theme_name} not found.")
        return
    
    with open(path, 'r', encoding='utf-8') as f:
        config = json.load(f)
        
    css_vars = "\n".join([f"    --{k}: {v};" for k, v in config.items()])
    css_out = f":root {{\n{css_vars}\n}}\n"
    
    out_path = os.path.join(ROOT, "sigma_theme_bundle.css")
    with open(out_path, 'w', encoding='utf-8') as f:
        f.write(css_out)
    
    print(f"💎 CSS Bundle generated at {out_path}")
    print("Agentic Note: Google Stitch UI must link this CSS shard.")

def run_customizer_demo():
    print("--- [ SIGMA CUSTOMIZER ] ---")
    print("1. Creating 'Midnight Cyberpunk' Theme...")
    save_theme("midnight_cyberpunk", default_theme)
    
    print("2. Creating 'NCERT Academic Mode' Theme (High Contrast, Low Strain)...")
    ncert_theme = default_theme.copy()
    ncert_theme.update({
        "sigma-bg": "#FFFFFF",
        "sigma-surface": "#F0F0F0",
        "sigma-border": "#CCCCCC",
        "sigma-accent-primary": "#0055FF",
        "sigma-text-main": "#111111",
        "sigma-text-muted": "#555555"
    })
    save_theme("ncert_academic", ncert_theme)
    
    print("3. Distilling CSS Payload...")
    generate_css_bundle("midnight_cyberpunk")

if __name__ == "__main__":
    run_customizer_demo()
