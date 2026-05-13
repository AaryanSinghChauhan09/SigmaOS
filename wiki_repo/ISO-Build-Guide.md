# ISO-Build-Guide

1

This document provides a technical guide for generating the first bootable SigmaOS ISO based on Arch Linux.

1

1

1

1

1

Install `archiso` on an Arch-based host:

1

sudo pacman -S archiso
cp -r /usr/share/archiso/configs/releng/ ~/sigmaos-iso

1

1

Add the core SigmaOS foundation packages to `packages.x86_64`:

1

1

1

1

Inject SigmaOS branding into the live environment:

1

1

Configure `calamares` modules for SigmaOS:

1

1

Execute the build script:

1

sudo mkarchiso -v -w /tmp/archiso-tmp -o ~/iso-output ~/sigmaos-iso

1

1

1

1

---

1

