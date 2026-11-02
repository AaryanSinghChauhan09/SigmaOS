# SigmaOS-plugin-marketplace

SigmaOS-plugin-marketplace is the plugin and extension marketplace for SigmaOS, providing a curated catalogue of applets, themes, extensions, and desktop widgets with one-click installation, ratings, screenshots, and version management. It connects to the SigmaOS content delivery network to browse and install community and first-party extensions, and integrates with SigmaOS-packages-kit for secure, verified installation workflows.

**Upstream reference projects:** [cinnamon-spices-applets](https://github.com/linuxmint/cinnamon-spices-applets)

## Build Instructions

```bash
# Clone the repository
git clone https://github.com/SigmaOS/SigmaOS-plugin-marketplace.git
cd SigmaOS-plugin-marketplace

# Install Python dependencies
pip install -r requirements.txt

# Run the marketplace
python3 src/main.py
```

Dependencies: `python3 >= 3.10`, `python3-gi`, `python3-requests`, `libgtk-4-dev`

## Upstream Reference

| Upstream | Concept absorbed |
|----------|-----------------|
| [Arch User Repository (AUR)](https://aur.archlinux.org/) | Trusted community catalogue with trust_score and votes; search/install model |
| [Omarchy Plugin Marketplace](https://github.com/basecamp/omarchy) | Decentralised community widget/extension discovery |
