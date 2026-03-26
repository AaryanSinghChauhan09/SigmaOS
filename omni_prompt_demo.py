import sys
import os

sys.path.insert(0, os.path.abspath("."))

from userland.system_api.ai_integration.omni_prompt_distributor import get_omni_prompt

def run_distributor_demo():
    print("💎 --- SigmaOS Sovereign Omni-Prompt Improvised Core --- 💎")
    
    # 1. Initialize the Distributor
    distributor = get_omni_prompt()
    distributor.initialize()

    # 2. User defines a custom model (Improvisation: Scalability)
    distributor.add_model("Local-Llama", "http://localhost:11434", "textarea")

    # 3. Define the Prompt
    user_prompt = "Explain the architectural advantages of an atomic micro-sharded OS compared to a monolithic kernel."

    # 4. Define Target Models
    targets = ["ChatGPT", "Claude", "Gemini", "Local-Llama"]

    # 5. Execute Distribution (No-Submit Enforcement)
    print(f"\n[USER-ACTION] Distributing Prompt to {len(targets)} models...")
    distributor.execute("DISTRIBUTE", prompt=user_prompt, models=targets)

    print("\n✅ Omni-Prompt Synced Across Nexus. User may now manually review and submit in each window.")

if __name__ == "__main__":
    run_distributor_demo()
