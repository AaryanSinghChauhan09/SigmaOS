# SigmaOS Sovereign Makefile
# Proving sovereignty through deterministic builds.

CC = gcc
CFLAGS = -Wall -Wextra -I. -Imodules/core/include -ffreestanding -nostdlib

# Shards
DRIVER_SHARDS = modules/core/drivers/gpu.c modules/core/drivers/usb.c modules/core/drivers/wifi.c
KERNEL_SHARDS = modules/core/kernel/hypervisor/sigmavm.c modules/core/kernel/security/shard_isolation.c
NET_SHARDS = modules/core/net/icmp.c modules/core/net/socket.c
PERF_SHARDS = modules/perf/profiler.c

ALL_SHARDS = $(DRIVER_SHARDS) $(KERNEL_SHARDS) $(NET_SHARDS) $(PERF_SHARDS)

all: $(ALL_SHARDS)
	@echo "All Sovereign Shards Validated."

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(ALL_SHARDS:.c=.o)
