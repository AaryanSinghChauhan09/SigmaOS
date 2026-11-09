# SigmaOS user programs — flat binaries for `exec <file>`
# Position-independent code only: exec loads at a runtime-chosen address.
# Output is a raw .bin (objcopy) whose first byte is the entry point.

CC      ?= gcc
AS      ?= gcc
OBJCOPY ?= objcopy
ASFLAGS := -m32 -c
LDFLAGS := -m32 -nostdlib -static -no-pie -Wl,--build-id=none -Wl,-e,_start

PROGS := hello cat rm mkdir touch mv append stat

.PHONY: all clean
all: $(addsuffix .bin,$(PROGS))

%.bin: %.elf
	$(OBJCOPY) -O binary $< $@

%.elf: %.o
	$(CC) $(LDFLAGS) $< -o $@

%.o: %.S
	$(CC) -m32 -c $< -o $@

clean:
	rm -f *.o *.elf *.bin

elfhello.elf: elfhello.c
	$(CC) -m32 -nostdlib -static -fno-stack-protector -fno-pie -no-pie -Os $< -o $@
