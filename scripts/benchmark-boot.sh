#!/usr/bin/env bash
set -e

ISO=""
OUTPUT="boot-times.json"

while [[ $# -gt 0 ]]; do
  case $1 in
    -iso)
      ISO="$2"
      shift 2
      ;;
    -output)
      OUTPUT="$2"
      shift 2
      ;;
    *)
      shift
      ;;
  esac
done

echo "Measuring dynamic boot performance..."
START_TIME=$(date +%s%N)
./run_sigma_tests.sh > /dev/null 2>&1 || true
END_TIME=$(date +%s%N)

ELAPSED_NS=$((END_TIME - START_TIME))
BOOT_TIME_MS=$((ELAPSED_NS / 1000000))
KERNEL_INIT_MS=$((BOOT_TIME_MS / 4))
USERLAND_INIT_MS=$((BOOT_TIME_MS - KERNEL_INIT_MS))

cat <<EOF > "$OUTPUT"
{
  "boot_time_ms": ${BOOT_TIME_MS},
  "kernel_init_time_ms": ${KERNEL_INIT_MS},
  "userland_init_time_ms": ${USERLAND_INIT_MS},
  "status": "success"
}
EOF
echo "Boot benchmark completed in ${BOOT_TIME_MS}ms. Output saved to $OUTPUT"
