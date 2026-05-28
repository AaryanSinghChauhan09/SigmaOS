# SigmaOS Science Spin — Researcher & Scientist Edition

The **SigmaOS Science** spin equips researchers, data scientists, and engineers with a complete computational science stack — from statistical computing and AI/ML to molecular simulation and high-performance numerical methods.

---

## 🔢 Mathematics & Statistics

| Tool | Purpose |
|------|---------|
| R + RStudio | Statistical computing & data analysis |
| GNU Octave | MATLAB-compatible numerical computing |
| SageMath | Pure mathematics, algebra, calculus |
| Maxima | Computer algebra system (CAS) |
| bc / units | Sovereign command-line math utilities |

## 🐍 Scientific Python Stack

All pre-installed in an isolated conda/uv-managed environment:

```
numpy scipy matplotlib pandas scikit-learn
seaborn plotly statsmodels sympy networkx
jupyter jupyterlab ipywidgets
```

## 🤖 AI & Machine Learning

| Framework | Backend |
|-----------|---------|
| PyTorch | CUDA / ROCm / CPU |
| TensorFlow | CPU / GPU |
| JAX | XLA-accelerated numerical computing |
| Hugging Face Hub | Pretrained model access |
| Sovereign EdgeML | SigmaOS native on-device inference |

## 🧬 Simulation & Modeling

- **OpenFOAM** — computational fluid dynamics (CFD)
- **GROMACS** — molecular dynamics simulation
- **Quantum ESPRESSO** — electronic structure calculation
- **ParaView** — scientific data visualization
- **VMD** — molecular visualization

## 📊 Data Visualization

- **Matplotlib / Seaborn** — Python-native plots
- **Plotly / Dash** — interactive web-based dashboards
- **Gnuplot** — sovereign command-line graphing
- **ROOT (CERN)** — high-energy physics data analysis

## 🧪 Laboratory & Instrumentation

- **LabPlot** — data acquisition & analysis
- **KiCad** — circuit design (for experimental apparatus)
- **QUCS** — circuit simulation

## 🗃 Data Management

- **DuckDB** — in-process SQL analytics
- **HDF5 / NetCDF** — scientific data formats
- **Zenodo** — sovereign research data publication integration

## ⚡ HPC & Parallel Computing

- **OpenMPI** — message passing for cluster workloads
- **OpenMP** — shared-memory parallelism
- **CUDA Toolkit** (optional) — NVIDIA GPU acceleration

---

## 🚀 Installation

```bash
sigma-spin install science
```

## 📚 See Also

- [Sovereign EdgeML](Intelligence.md)
- [WASI Compatibility Layer](Sovereign-Sandbox.md)
