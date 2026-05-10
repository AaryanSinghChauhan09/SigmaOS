# Σ SigmaOS Micro-Edition Container Build
# Absorbing Alpine Linux minimalism (musl + busybox)

FROM scratch

# L0: Core Lattice Shards
COPY kernel/core/SovereignHAL.so /lib/sigma/L0/
COPY kernel/core/SovereignPMM.so /lib/sigma/L0/

# L2: Security (PQC)
COPY kernel/core/security/SovereignPQC.so /lib/sigma/L2/

# L5: Industrial Utilities (SovereignBusyBox)
COPY userland/SovereignBusyBox.so /bin/busybox

# Symlinks for multi-call binary
RUN ["/bin/busybox", "ln", "-s", "/bin/busybox", "/bin/ls"]
RUN ["/bin/busybox", "ln", "-s", "/bin/busybox", "/bin/cat"]
RUN ["/bin/busybox", "ln", "-s", "/bin/busybox", "/bin/sh"]

# Entry Point: Sovereign Init
ENTRYPOINT ["/lib/sigma/L0/SovereignInit.so"]
