1


SigmaOS aims for broad hardware compatibility through its specialized **Hardware Abstraction Layer (HAL)** shards.


1


To fill technical gaps and ensure drivers work for almost every device, SigmaOS features a [Universal Linux Driver Compatibility Layer](Linux-Driver-Compat.md). This allows SigmaOS to seamlessly load and execute drivers packaged for various Linux distros (Debian, Fedora, Arch, etc.) by wrapping the Linux kernel ABI natively.


1


We primarily validate SigmaOS using **QEMU 7.0+** with the following configuration:


1


---
*To report hardware issues, please use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.yml).*

