# Per-App Network Policies

## 1. Default Posture: Deny All
Traditional operating systems allow any userland application to bind to a port or initiate outbound connections. SigmaOS flips this model. By default, applications have **no network access**.

## 2. Declarative Firewalls
Applications must declare their network requirements in their `sigpkg` manifest or via the SigmaInit service configuration.
- **Example:** A web browser package requests `OUTBOUND 80/tcp, 443/tcp`.
- **Example:** A database requests `INBOUND 5432/tcp`.

## 3. MicroVM Network Namespaces
When an application is launched, it is placed inside a `sigma_sandbox.rs` MicroVM.
- A virtual network interface (`veth`) is generated for that specific VM.
- Egress/Ingress iptables/nftables-equivalent rules are applied directly to that specific interface based on the declarative manifesto.
- If an app attempts to connect to an unauthorized port, the packet is instantly dropped at the hypervisor level.

## 4. WireGuard Profiles
For enterprise and zero-trust deployments, SigmaOS integrates WireGuard primitives natively.
- Individual MicroVMs can be bound exclusively to a WireGuard tunnel.
- This ensures that specific applications (e.g., an internal corporate dashboard) can only communicate over the encrypted VPN, and have zero access to the public internet or local LAN, completely isolating the threat surface.
