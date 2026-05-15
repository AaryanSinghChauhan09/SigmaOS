#ifndef SIGMA_INTERRUPT_HPP
#define SIGMA_INTERRUPT_HPP

#include "../../../include/libc/sigma_libc.h"

namespace sigma {
namespace hal {

// Abstract interrupt handler — every IRQ is its own module
class IInterruptHandler {
public:
    virtual ~IInterruptHandler() {}
    virtual int get_irq_number() const = 0;
    virtual void handle() = 0;
};

// Concrete: Timer IRQ handler (IRQ 0)
class TimerInterruptHandler : public IInterruptHandler {
public:
    int get_irq_number() const override { return 0; }
    void handle() override {
        sigma_kprint("[SigmaHAL] IRQ0: Timer interrupt handled. Scheduler tick.\n");
    }
};

// Concrete: Keyboard IRQ handler (IRQ 1)
class KeyboardInterruptHandler : public IInterruptHandler {
public:
    int get_irq_number() const override { return 1; }
    void handle() override {
        sigma_kprint("[SigmaHAL] IRQ1: Keyboard interrupt — scancode queued.\n");
    }
};

// Concrete: NIC IRQ handler (IRQ 11)
class NetworkInterruptHandler : public IInterruptHandler {
public:
    int get_irq_number() const override { return 11; }
    void handle() override {
        sigma_kprint("[SigmaHAL] IRQ11: NIC interrupt — DMA ring notified.\n");
    }
};

// IRQ Dispatcher: routes hardware interrupts to atomic handlers
class IRQDispatcher {
private:
    IInterruptHandler* handlers[3];
    int count;

public:
    IRQDispatcher() : count(0) {
        handlers[count++] = new TimerInterruptHandler();
        handlers[count++] = new KeyboardInterruptHandler();
        handlers[count++] = new NetworkInterruptHandler();
    }

    ~IRQDispatcher() {
        for (int i = 0; i < count; i++) delete handlers[i];
    }

    void dispatch(int irq) {
        for (int i = 0; i < count; i++) {
            if (handlers[i]->get_irq_number() == irq) {
                handlers[i]->handle();
                return;
            }
        }
        sigma_kprint("[SigmaHAL] Unhandled IRQ received.\n");
    }
};

} // namespace hal
} // namespace sigma

#endif // SIGMA_INTERRUPT_HPP
