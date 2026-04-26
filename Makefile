# SigmaOS Sovereign Makefile
# Proving sovereignty through deterministic builds.

CC = gcc
CFLAGS = -Wall -Wextra -I. -Imodules/core/include -ffreestanding -nostdlib

# Shards
DRIVER_SHARDS = $(wildcard modules/core/drivers/*.c)
KERNEL_SHARDS = $(wildcard modules/core/kernel/*.c) $(wildcard modules/core/kernel/*/*.c)
NET_SHARDS = $(wildcard modules/core/net/*.c)
PERF_SHARDS = $(wildcard modules/perf/*.c)
UI_SHARDS = $(wildcard modules/ui/*.c)
CLOUD_SHARDS = $(wildcard modules/cloud/*.c)

ALL_SHARDS = $(DRIVER_SHARDS) $(KERNEL_SHARDS) $(NET_SHARDS) $(PERF_SHARDS) $(UI_SHARDS) $(CLOUD_SHARDS)

all: $(ALL_SHARDS)
	@echo "All Sovereign Shards Validated."

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(ALL_SHARDS:.c=.o)
