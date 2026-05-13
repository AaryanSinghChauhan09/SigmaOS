# Getting-Started

1

Welcome to the SigmaOS Sovereign Lattice. This guide will help you set up your development environment and run your first SigmaOS instance.

1

Before you begin, ensure you have the following installed:

1

1

1

1

git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

1

1

The SigmaOS build system is modular and shard-based.

1

1

make all

1

1

We provide a specialized boot script that handles ISO creation and QEMU orchestration.

1

1

./qemu-boot.sh

1

To view the kernel logs in real-time:

1

tail -f serial.log

1

1

If you want to interact with the experimental web-based UI:

1

npm install
node server.js

1

1

1

1

1

---

1

