# Upstream Study: OpenBSD Proactive Security & Secure Defaults

1. **Upstream Problem**: Proactive vulnerability mitigation, W^X memory allocation, and privilege separation.
2. **Data Structure & Control Flow**: Strict pledge/unveil checks at system call entries; privilege-separated child daemons.
3. **Security & Failure Assumptions**: Default-closed system access; immediate process SIGKILL on policy violation.
4. **Direct Adoption**: Secure defaults policy, W^X memory guarantees, and proactive audit culture.
5. **Rust/SigmaOS Adaptation**: Monotonic process privilege guard in safe Rust preventing capability escalation.
6. **Exit Test Criteria**: Execution attempt on writable memory page triggers immediate segmentation fault / drop.
7. **Licensing & Attribution**: ISC / BSD License.
