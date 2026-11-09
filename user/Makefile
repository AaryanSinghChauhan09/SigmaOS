# SigmaOS user programs — flat binaries for `exec <file>`
# Position-independent code only: exec loads at a runtime-chosen address.
# Output is a raw .bin (objcopy) whose first byte is the entry point.

CC      ?= gcc
AS      ?= gcc
OBJCOPY ?= objcopy
ASFLAGS := -m32 -c
LDFLAGS := -m32 -nostdlib -static -no-pie -Wl,--build-id=none -Wl,-e,_start

.PHONY: all clean
all: hello.bin

hello.bin: hello.elf
	$(OBJCOPY) -O binary $< $@

hello.elf: hello.o
	$(CC) $(LDFLAGS) $< -o $@

hello.o: hello.S
	$(CC) -m32 -c $< -o $@

clean:
	rm -f *.o *.elf *.bin
