"""
SigmaOS NCERT Standalone Utilities Registry
Registry for non-interactive form tools (launchers)
"""

class StandaloneUtils:
    TITLE = "NCERT Interactive Tools & Utilities"
    EXP_DATA = {
        "Chemical Balancer": "launch:ncert_chem_balancer:ChemBalancer",
        "Logic Gate Simulator": "launch:ncert_logic_circuit:LogicSimulator",
        "Sets Venn Visualizer": "launch:ncert_venn_visualizer:VennVisualizer",
        "Periodic Table View": "launch:ncert_periodic_table:PeriodicTable",
        "Math Identity Visual": "launch:ncert_math_visualizer:IdentityVisualizer",
    }

UTILS_REGISTRY = {
    "Interactive Tools": StandaloneUtils
}
