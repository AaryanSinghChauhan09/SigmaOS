# SigmaOS shared logging helpers
# Modeled on Omarchy 4.0.5 install/helpers/logging.sh
# Provides consistent logging across all install/provision scripts.

set -euo pipefail

log_info()  { echo "[sigmaos] [INFO]  $*" >&2; }
log_step()  { echo "[sigmaos] [STEP]  $*" >&2; }
log_warn()  { echo "[sigmaos] [WARN]  $*" >&2; }
log_error() { echo "[sigmaos] [ERROR] $*" >&2; }
log_success() { echo "[sigmaos] [ OK ]  $*" >&2; }

# Run a script with logging, like Omarchy's run_logged.
run_logged() {
  local script="$1"
  log_step "running: ${script}"
  bash "${script}" || { log_warn "${script} exited non-zero"; return 1; }
  log_success "${script} completed"
}
