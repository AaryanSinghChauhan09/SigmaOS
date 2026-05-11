1



1



1



1



1
<<<<<<< HEAD



1


make all


=======

make all

>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
1



1


The `SHARDS.manifest` file lists all 600+ shard `.cpp` files compiled by the Makefile. To add a new shard:



1. Create your `.cpp` file under the appropriate module directory.
2. Add its path to `SHARDS.manifest`.



3. Run `make`.


1



1

<<<<<<< HEAD

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
-std=c++17 -ffreestanding -fno-exceptions -fno-rtti
-nostdlib -nostdinc++ -Wall -Wextra -Wpedantic
-I include -I include/core -I include/libc

<<<<<<< HEAD

=======
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
1


> [!IMPORTANT]
> **No stdlib allowed.** Never `#include <iostream>`, `<string>`, or any STL header. Use `SovereignLibC.h` and `SigmaOOP.hpp` exclusively.


1



1

<<<<<<< HEAD

qemu-system-aarch64 -machine raspi4b -kernel sigma_os.elf -serial stdio


=======
qemu-system-aarch64 -machine raspi4b -kernel sigma_os.elf -serial stdio

>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
1



1


The GitHub Actions matrix build runs `make ARCH=aarch64` and `make ARCH=x86_64` on every push to `main`. See `.github/workflows/` for full configuration.


1


The `compile_flags.txt` passes `-nostdinc++` to clangd, ensuring the IDE also enforces the zero-stdlib rule during development.

