# Hardware-Support


SigmaOS aims for broad hardware compatibility through its specialized **Hardware Abstraction Layer (HAL)** shards.


To fill technical gaps and ensure drivers work for almost every device, SigmaOS features a [Universal Linux Driver Compatibility Layer](Linux-Driver-Compat). This allows SigmaOS to seamlessly load and execute drivers packaged for various Linux distros (Debian, Fedora, Arch, etc.) by wrapping the Linux kernel ABI natively.


We primarily validate SigmaOS using **QEMU 7.0+** with the following configuration:



---
*To report hardware issues, please use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.yml).*
