# SigmaOS-NVIDIA

SigmaOS-NVIDIA provides NVIDIA GPU driver integration and PRIME switching support for SigmaOS, managing driver installation, CUDA setup, and hybrid graphics switching between integrated and discrete GPUs. It simplifies the traditionally complex process of enabling NVIDIA hardware on Linux by automating driver selection, PRIME profile configuration, and runtime power management for both desktop and laptop systems.

**Upstream reference projects:** [nvidia-prime](https://github.com/Bumblebee-Project/nvidia-prime)

## Build Instructions

```bash
# Clone the repository
git clone https://github.com/SigmaOS/SigmaOS-NVIDIA.git
cd SigmaOS-NVIDIA

# Run the NVIDIA driver manager
bash src/main.sh
```

Dependencies: `nvidia-driver`, `nvidia-prime`, `nvidia-settings`
