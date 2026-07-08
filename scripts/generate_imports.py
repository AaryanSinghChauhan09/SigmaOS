import os
import re
import subprocess
import json

raw_text = """
120 Suggested imports (numbered, grouped by subsystem)
Kernel & Drivers (1–30)
i915 Intel DRM updates — torvalds/linux — integrate latest stability and power patches.
AMDGPU kernel improvements — torvalds/linux — absorb perf/power fixes and firmware packaging.
Nouveau fixes and packaging — mesa/torvalds/linux — backport stability patches for common GPUs.
Broadcom Wi‑Fi firmware packaging — Debian/Fedora patches — signed firmware loader and secure packaging.
Realtek Wi‑Fi backports — Arch/Fedora — backport stable fixes for consumer chipsets.
Intel wireless power management — linux/firmware — reduce suspend/resume issues and power draw.
BlueZ Bluetooth improvements — bluez — integrate pairing and audio stability patches.
Thunderbolt authorization flow — Fedora/Arch — implement secure device whitelisting UI.
ACPI suspend/resume fixes — linux — reduce resume failures on popular laptops.
ALSA + PipeWire kernel hooks — Fedora/Arch — low‑latency audio path tuning.
NVMe driver tuning defaults — linux — SSD queue and scheduler presets for responsiveness.
Signed ZFS module packaging — Debian/Arch — reproducible, signed ZFS kernel modules.
Btrfs auto‑snapshot tooling — Fedora/Arch — scheduled scrub and snapshot automation.
eMMC/SD reliability patches — linux — improve embedded device storage reliability.
GPU power/perf governors — Fedora — persona profiles for battery vs performance.
TPM driver + tpm2 tools — Fedora/Debian — attestation and sealed storage flows.
Secure Boot shim + key mgmt — Debian/Fedora — guided key enrollment and verification.
Kernel livepatch integration — Canonical/Fedora — hotpatch CVE fixes in production.
Network offload defaults (RSS/GRO/LRO) — linux — NIC tuning for throughput/latency.
Regulatory DB packaging — Debian — correct Wi‑Fi regulatory defaults per region.
V4L2 camera driver fixes — Arch/Debian — firmware packaging and privacy toggles.
Fingerprint sensor support (fprintd) — Fedora/Arch — secure enrollment and UX.
ROCm/CUDA compute packaging — Fedora/Arch — signed compute stacks for AI workloads.
Early serial console improvements — linux — robust early boot logging and crash capture.
fscrypt per‑file encryption hooks — Debian — default encrypted home support.
Enterprise NIC drivers packaging — Fedora — Intel/Realtek server NICs with tests.
SDR and USB radio drivers — Kali/Arch — optional SigmaSec modules for researchers.
ARM SoC board support packages — linux/nixos — reproducible images for SBCs.
Thermal and fan control drivers — Fedora — thermal profiles and user controls.
Structured kernel crash telemetry — linux — automated OOPS/panic reporting to CI.
Packaging, Build & Reproducibility (31–55)
sigpkg manifest schema — NixOS/Nixpkgs inspiration — declarative metadata with SBOM fields.
GPG‑signed package repo tooling — Fedora — signing, verification, and key rotation scripts.
Binary delta update algorithm — OSTree/Fedora Silverblue — reduce update sizes.
Optional immutable root (OSTree) — Silverblue — layered packages on immutable base.
Nix‑style declarative system manifests — NixOS — reproducible system configuration model.
Deterministic build farm orchestration — Nix/Fedora — containerized reproducible builders.
SBOM generation pipeline — Nix/Guix — SBOMs for every artifact in CI.
Atomic update + rollback hooks — Nix/OSTree — safe upgrades with rollback.
PKGBUILD/RPM/DEB → sigpkg converters — Arch/Fedora/Debian — automated conversion scripts.
Cross‑compile toolchain recipes — Debian/Nix — reproducible embedded builds.
Package vulnerability scanning in CI — Debian/Fedora — CVE checks pre‑merge.
Local offline mirror manager — Debian/Arch — create air‑gapped mirrors for installs.
Installer image signing & verification — Fedora — ensure ISO integrity at boot.
Persona bundle packaging — Fedora/Ubuntu — curated Dev/Data/Security bundles.
Build cache and CDN mirrors — Arch/Fedora — accelerate installs and updates.
Package provenance metadata — Nix — commit hashes and builder IDs in packages.
Automated backport pipeline — Debian — backport critical fixes to LTS images.
Hermetic build sandboxes — Nix/Guix — isolated, reproducible build environments.
Package linting rules and CI hooks — Arch/Debian — enforce packaging quality.
Signed kernel + initramfs builder — Fedora — reproducible boot artifacts.
Delta patch generation for kernel modules — Fedora — smaller driver updates.
Image build CI templates — Fedora/Nix — reproducible ISO/OCI pipelines.
Local package signing verification at install — Fedora — enforce provenance on client.
Automated SBOM publishing — Nix — publish SBOMs to artifact registry.
Package dependency graph visualizer — Arch/Nix — maintainers debug dependency bloat.
Installer, Imaging & First‑Run (56–70)
Modular Calamares installer modules — Calamares/Arch — persona selection, encryption, drivers.
Encrypted defaults (LUKS + TPM) — Ubuntu/Fedora — encrypted home and disk by default.
Secure Boot enrollment UI — Debian/Fedora — guided key enrollment for users.
Persona bundle installer flow — Ubuntu/Fedora — preconfigured persona selection.
Network installer with offline bundles — Debian — offline persona provisioning.
Automated HCL detection during install — Arch/Fedora — preselect drivers and firmware.
Installer rollback snapshot — OSTree — snapshot before major changes.
First‑run privacy & telemetry opt‑in — Ubuntu — transparent telemetry choices.
Automated driver test harness in installer — Fedora — run hardware tests on first boot.
PXE and enterprise imaging support — Debian — network boot and mass deploy.
Migration assistant (Windows/macOS) — Ubuntu — import settings and data.
Accessibility first‑run flow — Ubuntu — screen reader and high‑contrast setup.
Installer image signing and verification — Fedora — ensure image integrity.
Rollbackable upgrade path — Silverblue/Nix — safe OS upgrades with rollback.
Offline documentation bundle — Debian — ship full docs for air‑gapped users.
Desktop, Compositor & UX (71–90)
Wayland‑first Zenith compositor — Fedora/GNOME Wayland patches — low‑latency compositor.
XWayland compatibility tuning — Fedora — ensure legacy app performance.
PipeWire audio defaults and portals — Fedora — low‑latency audio and sandboxing.
Fractional HiDPI scaling fixes — Fedora/Ubuntu — crisp UI on modern displays.
Session restore and workspace persistence — GNOME/KDE — robust session management.
Unified settings with persona profiles — GNOME — switch persona presets centrally.
Flatpak‑style app sandboxing + portals — Flatpak/Fedora — secure app resource access.
Native file manager with previews and remote mounts — GNOME/KDE — productivity features.
Integrated screenshot + annotation tool — GNOME — built‑in productivity tool.
Window tiling and keyboard workflows — i3/tiling extensions — power user features.
Accessibility suite (screen reader, magnifier) — Ubuntu — ship and configure by default.
App store with signed apps and reviews — Fedora/Ubuntu — curated, audited marketplace.
Power profile UI and governor switching — Fedora — per‑persona power/perf presets.
Session sandbox for untrusted web apps — Qubes/containers — disposable browser sessions.
Native PDF editor and annotation — Evince/Okular — integrated document workflows.
Unified notifications and Do Not Disturb — GNOME — consistent UX.
Remote desktop with Wayland streaming — Fedora — secure low‑latency remote access.
Theme/HIG and developer guidelines — GNOME — consistent app design language.
Compositor GPU buffer reuse — Wayland patches — reduce CPU/GPU overhead.
Compositor performance benchmark suite — Arch/Fedora — automated latency/frame tests.
Security, Sandboxing & Forensics (91–115)
Firecracker microVM per‑app sandboxing — Qubes/Firecracker — strong isolation for untrusted apps.
gVisor container isolation option — gVisor — alternative sandboxing for workloads.
SELinux/AppArmor default profiles — Fedora/Debian — enforce least privilege for services.
TPM attestation workflows — Fedora — device attestation and sealed secrets.
Hardware‑backed key mgmt (PKCS#11) — Fedora — integrate with user apps.
Per‑app network policy engine — Fedora/nftables — per‑app firewall and WireGuard profiles.
Suricata IDS packaged with default rules — Kali/Fedora — network intrusion detection.
Forensics toolkit bundle — Kali — Volatility, Autopsy, Sleuth Kit preconfigured.
Secure update proofs — Nix/Fedora — reproducible signed update proofs for clients.
Disposable browser VM template — Qubes — ephemeral browsing environments.
Auditd syscall monitoring templates — Fedora — baseline audit rules.
Secure logging with tamper evidence — systemd/journal + signing — forensic integrity.
Encrypted swap and zram policies — Debian/Fedora — secure memory defaults.
Secure default SSH hardening — Debian/Fedora — modern algorithms and rate limits.
Secrets manager integration (Vault) — HashiCorp patterns — secrets lifecycle and rotation.
Automated incident response playbooks — Kali/Qubes — runnable playbooks in Wiki.
Hardware isolation test harness — Qubes — validate microVM isolation on HCL devices.
Container image signing (Notary/OCI) — Docker/Notary — verify container provenance.
App capability tokens model — Qubes-like — fine‑grained capability model for apps.
Tamper‑resistant boot chain enforcement — Fedora — signed bootloader, kernel, initramfs.
CVE auto‑patch pipeline — Fedora/Canonical — prioritized hotpatching and staged rollouts.
Privacy dashboard and telemetry controls — Ubuntu/Fedora — transparent telemetry management.
Hardware-backed disk encryption keys — Debian/Fedora — LUKS + TPM sealing integration.
Network segmentation templates — Fedora — persona network isolation blueprints.
Secure default browser sandboxing — Fedora/Kali — site isolation and plugin restrictions.
AI, Data Science, Observability & Dev Tools (116–135)
ONNX runtime packaging + quantized models — Nix/Fedora — local offline inference runtime.
Signed model store + cache — Hugging Face/community — offline model marketplace with provenance.
JupyterLab secure sandbox — Debian/Fedora — notebook isolation and resource limits.
MLflow experiment tracking integration — community — reproducible ML pipelines.
DVC dataset versioning templates — community — dataset provenance and CI hooks.
Prometheus node exporter defaults — Fedora — observability baseline.
Grafana dashboards + persona templates — Fedora — prebuilt dashboards for Dev/Data/Sec.
OpenTelemetry + Jaeger tracing — Fedora — distributed tracing for services.
Local model quantization toolchain — Nix/Fedora — reduce model size for offline use.
Prebuilt legal/education datasets — community — offline corpora for bundles.
Rust dev toolchain templates — Fedora/Arch — reproducible Rust project templates and CI.
VSCodium packaging + curated extensions — Arch/Fedora — audited IDE experience.
Static analysis + fuzzing harnesses in CI — Fedora/Arch — security testing for native code.
Benchmark suite (boot, compositor, package install) — Arch/Fedora — automated performance regression tests.
CLI NL→CLI agent dry‑run sandbox — community — safe command preview and approval flow.
Prebuilt reproducible datasets for ML — community — legal, education, privacy datasets.
Dev sandbox templates (microVM + container) — Qubes/Nix — reproducible dev environments.
Model provenance SBOMs — Nix — SBOMs for models and data artifacts.
Local LLM inference adapter (ONNX + quant) — community — small LLMs for NL→CLI features.
Automated telemetry anonymization pipeline — privacy projects — safe telemetry for product improvement.
"""

def slugify(s):
    s = s.lower()
    s = re.sub(r'[^a-z0-9]+', '-', s)
    return s.strip('-')

lines = raw_text.strip().split('\n')
items = []
current_area = "general"

for line in lines:
    line = line.strip()
    if not line:
        continue
    
    # Check for area headers
    if "(" in line and ")" in line and "-" in line.split("(")[-1] or "–" in line.split("(")[-1]:
        current_area = slugify(line.split("(")[0])
        continue
        
    parts = re.split(r'\s+[—\-]\s+', line)
    if len(parts) >= 3:
        feature = parts[0]
        source = parts[1]
        note = parts[2]
        items.append({
            "feature": feature,
            "source": source,
            "note": note,
            "area": current_area
        })

print(f"Found {len(items)} items to process.")

os.makedirs('docs/issues', exist_ok=True)
md_content = "# SigmaOS Third-Party Imports\\n\\n| Feature | Source | Note |\\n|---|---|---|\\n"

for idx, item in enumerate(items):
    f_slug = slugify(item['feature'])[:40]
    branch_name = f"work/{item['area']}/{f_slug}"
    
    # Create issue file
    issue_filename = f"docs/issues/import_{idx+1:03d}_{f_slug}.md"
    with open(issue_filename, 'w', encoding='utf-8') as f:
        f.write(f"# {item['feature']}\\n\\n")
        f.write(f"**Area**: {item['area']}\\n")
        f.write(f"**Source**: {item['source']}\\n")
        f.write(f"**Note**: {item['note']}\\n\\n")
        f.write(f"## Checklist\\n")
        f.write(f"- [ ] Reproducible build job\\n")
        f.write(f"- [ ] Tests\\n")
        f.write(f"- [ ] SBOM Generation\\n")
        f.write(f"- [ ] Documentation entry\\n")
        f.write(f"- [ ] Reviewer sign-off\\n")
        
    md_content += f"| {item['feature']} | {item['source']} | {item['note']} |\\n"
    
    # Create git branch
    try:
        subprocess.run(["git", "branch", branch_name, "main"], check=True, capture_output=True)
    except subprocess.CalledProcessError as e:
        print(f"Failed to create branch {branch_name}: {e.stderr.decode()}")

with open('docs/third_party_imports.md', 'w', encoding='utf-8') as f:
    f.write(md_content)

print("Generation complete!")
