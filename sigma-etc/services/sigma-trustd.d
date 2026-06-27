# /sigma/etc/services/sigma-trustd.d — PQ certificate authority daemon

type              = process
command           = /sigma/sbin/sigma-trustd
smooth-recovery   = true
restart           = true

# trustd depends only on TPM2 device availability
# If TPM2 isn't present: trustd starts but falls back to file-based key
waits-for         = sigma-tpm2

# Trustd must be ready before anything that needs mTLS
before            = sigma-apid
before            = sigma-healthd
before            = sigma-vault

ready-notification = pipefd:3
start-timeout     = 10
ready-timeout     = 20

max-restarts      = 3
restart-window-s  = 60
on-give-up        = kill-contract
