import os

suites = [
    {"id": "S11", "name": "Virtualization"},
    {"id": "S12", "name": "Ecosystem"},
    {"id": "S13", "name": "Sentience"},
    {"id": "S14", "name": "Transcendence"},
    {"id": "S15", "name": "DevNexus"},
    {"id": "S16", "name": "SoulMolding"},
    {"id": "S17", "name": "BioNexus"},
    {"id": "S18", "name": "QuantumLink"},
    {"id": "S19", "name": "SelfEvolution"},
    {"id": "S20", "name": "Interconnect"},
    {"id": "S21", "name": "EternalState"},
    {"id": "S22", "name": "SimulationNexus"},
    {"id": "S23", "name": "OmniNexus"},
    {"id": "S24", "name": "GlobalDebugger"},
    {"id": "S25", "name": "ZeroKernel"},
    {"id": "S26", "name": "OmniFabric"},
    {"id": "S27", "name": "NeuralLink"},
    {"id": "S28", "name": "OmniBus"},
    {"id": "S29", "name": "LatticeMerge"},
    {"id": "S30", "name": "Supremacy"},
    {"id": "S31", "name": "GlobalGovernance"},
    {"id": "S32", "name": "UnifiedSovereignty"},
    {"id": "S33", "name": "TerminalFulfillment"}
]

base_path = "suites"
for suite in suites:
    dir_name = f"{suite['id']}_{suite['name']}"
    path = os.path.join(base_path, dir_name)
    if not os.path.exists(path):
        os.makedirs(path)
        print(f"Created: {path}")
