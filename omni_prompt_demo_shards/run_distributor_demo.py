import sys
import os
from userland.system_api.ai_integration.omni_prompt_distributor import get_omni_prompt


def run_distributor_demo():
    print('💎 --- SigmaOS Sovereign Omni-Prompt Improvised Core --- 💎')
    distributor = get_omni_prompt()
    distributor.initialize()
    distributor.add_model('Local-Llama', 'http://localhost:11434', 'textarea')
    user_prompt = 'Explain the architectural advantages of an atomic micro-sharded OS compared to a monolithic kernel.'
    targets = ['ChatGPT', 'Claude', 'Gemini', 'Local-Llama']
    print(f'\n[USER-ACTION] Distributing Prompt to {len(targets)} models...')
    distributor.execute('DISTRIBUTE', prompt=user_prompt, models=targets)
    print('\n✅ Omni-Prompt Synced Across Nexus. User may now manually review and submit in each window.')