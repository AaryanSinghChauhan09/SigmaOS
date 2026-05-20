# ISO-Build-Guide


This document provides a technical guide for generating the first bootable SigmaOS ISO based on Arch Linux.






Install `archiso` on an Arch-based host:


sudo pacman -S archiso
cp -r /usr/share/archiso/configs/releng/ ~/sigmaos-iso



Add the core SigmaOS foundation packages to `packages.x86_64`:





Inject SigmaOS branding into the live environment:



Configure `calamares` modules for SigmaOS:



Execute the build script:


sudo mkarchiso -v -w /tmp/archiso-tmp -o ~/iso-output ~/sigmaos-iso





---

