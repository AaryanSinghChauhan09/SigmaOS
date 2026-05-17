# DRIVER PORTING PIPELINE

1

To rapidly expand hardware compatibility, SigmaOS employs a structured driver porting pipeline that leverages our [Universal Linux Driver Compatibility Layer](Linux-Driver-Compat.md).

1

1

We prioritize hardware categories essential for industrial and consumer use:

1. **Network** (Intel/Realtek Wi-Fi, Ethernet)

2. **Storage** (NVMe, SATA AHCI)

3. **Graphics** (Intel, AMD, NVIDIA basic modesetting)

4. **Input** (USB HID)

1

Instead of rewriting complex drivers from scratch (which can take years), we use our ABI compatibility shims (`SovereignLinuxCompat`) to load and execute GPL-compatible upstream Linux drivers natively within the SigmaOS `SovereignSandbox`.

1

1. **Request**: Users submit a hardware request using the [Driver Request Issue Template](https://github.com/AaryanSinghChauhan09/SigmaOS/issues/new/choose).

2. **Branching**: We maintain branches per hardware family (e.g., `feature/hw-realtek-wifi`).

3. **Porting**: The community maps the required Linux kernel symbols to the SigmaOS HAL.

4. **Testing**: The `Driver Porting CI` GitHub Action automatically compiles the driver and runs regression tests in QEMU.

1

1

1

1

We strongly encourage developers to submit patches for unsupported hardware!

1. Check the [Supported Hardware Matrix](Hardware-Support.md).

2. Find an open Driver Request issue.

3. Submit a Pull Request targeting the `drivers/linux_distros/` compatibility modules.
 