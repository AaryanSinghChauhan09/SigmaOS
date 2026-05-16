import os
import json

CATEGORIES = {
    "UX_Accessibility": {
        "competitors": ["Elementary", "Zorin"],
        "base_ideas": ["App Store GUI", "Screen Reader", "Adaptive UI Scaling", "Theme Engine", "Workspace Layouts"],
        "variants": ["Dark", "High-Contrast", "Minimal", "Immersive", "VR-Ready", "Industrial", "Focus", "Legacy", "Modern", "Quantum"]
    },
    "ARM_IoT": {
        "competitors": ["RPi-Distro"],
        "base_ideas": ["GPIO Manager", "Sensor Toolkit", "IoT Shard Profile", "Robotics Control", "Edge AI Inference"],
        "variants": ["Camera", "Motion", "Temp", "Actuator", "Drone", "Robot", "Smart-Home", "Factory", "Mesh", "Low-Power"]
    },
    "Gaming": {
        "competitors": ["SteamOS"],
        "base_ideas": ["GPU Optimizer", "Controller Manager", "Game Mode Scheduler", "VR/AR Shard", "Cloud Gaming Integration"],
        "variants": ["Low-Latency", "4K-Ready", "Vulkan-Opt", "Ray-Tracing", "Direct-Storage", "Haptic", "Spatial-Audio", "eSports", "Casual", "Retro"]
    }
}

def generate_ideas():
    output_dir = "wiki_repo/Generated-Ideas"
    os.makedirs(output_dir, exist_ok=True)
    
    root_content = "# Σ SigmaOS: The Lattice of Billions (Generated Ideas)\n\n"
    root_content += "This directory contains algorithmically generated ideas absorbed from competitor distributions.\n\n"
    
    for cat, data in CATEGORIES.items():
        cat_file = f"{output_dir}/{cat}.md"
        content = f"# {cat} Idea Expansion\n\n"
        content += f"Absorbed from: {', '.join(data['competitors'])}\n\n"
        content += "| Base Idea | Variant | Expansion (Lattice Shard) |\n"
        content += "| :--- | :--- | :--- |\n"
        
        for base in data['base_ideas']:
            for variant in data['variants']:
                shard_name = f"Sovereign{cat}{variant}{base.replace(' ', '')}"
                content += f"| {base} | {variant} | `{shard_name}` |\n"
        
        with open(cat_file, "w", encoding="utf-8") as f:
            f.write(content)
        
        root_content += f"- [{cat} Ideas]({cat}.md)\n"

    with open("wiki_repo/Lattice-of-Billions.md", "w", encoding="utf-8") as f:
        f.write(root_content)

if __name__ == "__main__":
    generate_ideas()
    print("Σ Idea Expansion Complete. Billions of ideas (simulated) generated.")
