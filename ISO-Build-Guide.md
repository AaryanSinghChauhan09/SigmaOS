1


This document provides a technical guide for generating the first bootable SigmaOS ISO based on Arch Linux.


1



1



1



1

<<<<<<< HEAD


1

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f

Install `archiso` on an Arch-based host:


1

<<<<<<< HEAD

sudo pacman -S archiso
cp -r /usr/share/archiso/configs/releng/ ~/sigmaos-iso


=======
sudo pacman -S archiso
cp -r /usr/share/archiso/configs/releng/ ~/sigmaos-iso

>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
1



1


Add the core SigmaOS foundation packages to `packages.x86_64`:


1



1

<<<<<<< HEAD


1



1

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f

Inject SigmaOS branding into the live environment:


1



1


Configure `calamares` modules for SigmaOS:


1



1


Execute the build script:


1

<<<<<<< HEAD

sudo mkarchiso -v -w /tmp/archiso-tmp -o ~/iso-output ~/sigmaos-iso


1



1



1



1

=======
sudo mkarchiso -v -w /tmp/archiso-tmp -o ~/iso-output ~/sigmaos-iso

1



1



1

>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f

---


1

