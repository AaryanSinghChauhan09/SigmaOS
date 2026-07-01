# SigmaOS — 1000+ Ideas Backlog

> Canonical source: [docs/IDEAS_1000.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/IDEAS_1000.md)

Living document — ~500 ideas across 7 categories, growing toward 1000+.
Every release adds ideas; contributors add more via GitHub Discussions.

---

## Categories

| Category | Ideas | Sub-themes |
|---|---|---|
| 🖥️ OS / Core System | ~100 | Kernel architectures, boot, virtualization, cloud images, packages, multi-format, distributed |
| 🔧 Drivers | ~75 | GPU (Intel/AMD/NVIDIA), Wi-Fi/BT, storage, peripherals, experimental |
| 🔒 Security | ~75 | Sandboxing, encryption, access control, network security, reproducibility |
| 🛠️ Tools | ~90 | Developer SDK, system utilities, networking, productivity, media, cloud sync |
| 🎨 Design | ~50 | Brand, desktop, accessibility, themes, animations |
| 🖼️ UI | ~40 | DE components, window manager, mobile UI, widgets |
| 🌟 UX | ~50 | Onboarding, docs, community, performance, privacy |
| 🤖 AI/ML | ~20 | On-device inference, NPU HAL, model packaging, sigma-ai |

---

## How to Add an Idea

1. Open a [GitHub Discussion](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions) with the `idea` label.
2. Or open a PR: add to `docs/IDEAS_1000.md`, numbered sequentially.
3. One line per idea — detail goes in a separate spec doc.

---

## Top 10 Highest-Impact Ideas Right Now

1. Bootable ISO (`make iso`) — unblocks everything
2. sigma-sh working REPL — usability baseline
3. sigma-pkg local install — package ecosystem entry
4. USB HID keyboard driver — hardware baseline
5. VESA/GOP framebuffer driver — display baseline
6. Formal verification of MM + IPC (seL4-inspired)
7. Content-addressed package store (Nix-inspired)
8. On-device TinyLlama inference (sigma-ai)
9. Smithay-inspired compositor (Rust, cleanroom)
10. QEMU hardware CI matrix (real driver testing)

---

*See the [full ideas list](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/IDEAS_1000.md) on GitHub.*
