// SigmaOS — sigma-libc-io: Native I/O Operations
// Modularised from: SovereignLibC.c
// USP: Pluggable OOP I/O backend for kernel print routines

#ifndef SIGMA_LIBC_IO_HPP
#define SIGMA_LIBC_IO_HPP

namespace sigma {
namespace libc {

class IBackendIO {
public:
    virtual ~IBackendIO() = default;
    virtual void write_char(char c) = 0;
    virtual void write_string(const char* str) = 0;
};

// VGA Text Mode backend
class VGAConsoleIO : public IBackendIO {
private:
    volatile char* video_memory = (volatile char*)0xB8000;
    int cursor_x = 0;
    int cursor_y = 0;

public:
    void write_char(char c) override {
        if (c == '\n') {
            cursor_x = 0;
            cursor_y++;
        } else {
            int offset = (cursor_y * 80 + cursor_x) * 2;
            video_memory[offset] = c;
            video_memory[offset + 1] = 0x07; // Light grey on black
            cursor_x++;
            if (cursor_x >= 80) {
                cursor_x = 0;
                cursor_y++;
            }
        }
    }

    void write_string(const char* str) override {
        while (*str) {
            write_char(*str++);
        }
    }
};

// Serial Port backend
class SerialIO : public IBackendIO {
private:
    unsigned short port = 0x3F8; // COM1

    void outb(unsigned short port, unsigned char val) {
#if defined(__x86_64__) || defined(__i386__)
        __asm__ __volatile__("outb %0, %1" : : "a"(val), "Nd"(port));
#endif
    }

public:
    void write_char(char c) override {
        outb(port, c);
    }

    void write_string(const char* str) override {
        while (*str) {
            write_char(*str++);
        }
    }
};

class KernelPrint {
private:
    IBackendIO* backend;

public:
    KernelPrint(IBackendIO* backend_io) : backend(backend_io) {}

    void print(const char* msg) {
        if (backend) backend->write_string(msg);
    }
};

} // namespace libc
} // namespace sigma

#endif // SIGMA_LIBC_IO_HPP
