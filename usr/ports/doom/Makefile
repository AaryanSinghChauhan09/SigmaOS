CC=gcc
CFLAGS=-ffreestanding -nostdlib -mno-red-zone -I../../include
OBJS=sigma_doom.o doomgeneric.o

doom: $(OBJS)
	$(CC) $(CFLAGS) -o doom $(OBJS) -static

clean:
	rm -f *.o doom
