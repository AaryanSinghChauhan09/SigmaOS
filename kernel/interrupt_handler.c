/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Advanced Interrupt Handler
 * ================================
 * Object-Oriented Interrupt Management with SOLID Principles
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

// x86_64 interrupt vectors
#define IDT_ENTRIES 256
#define IRQ_BASE 32
#define SYSCALL_VECTOR 0x80

// Interrupt types
typedef enum {
    INTERRUPT_TYPE_EXCEPTION = 0,
    INTERRUPT_TYPE_IRQ = 1,
    INTERRUPT_TYPE_SOFTWARE = 2,
    INTERRUPT_TYPE_NMI = 3
} InterruptType;

// Exception codes
typedef enum {
    EXC_DIVIDE_BY_ZERO = 0,
    EXC_DEBUG = 1,
    EXC_NMI = 2,
    EXC_BREAKPOINT = 3,
    EXC_OVERFLOW = 4,
    EXC_BOUND_RANGE = 5,
    EXC_INVALID_OPCODE = 6,
    EXC_DEVICE_NOT_AVAILABLE = 7,
    EXC_DOUBLE_FAULT = 8,
    EXC_INVALID_TSS = 10,
    EXC_SEGMENT_NOT_PRESENT = 11,
    EXC_STACK_SEGMENT_FAULT = 12,
    EXC_GENERAL_PROTECTION = 13,
    EXC_PAGE_FAULT = 14,
    EXC_X87_FPU_ERROR = 16,
    EXC_ALIGNMENT_CHECK = 17,
    EXC_MACHINE_CHECK = 18,
    EXC_SIMD_FP_EXCEPTION = 19
} ExceptionCode;

// OOP: Interrupt Handler Interface (Strategy Pattern)
typedef struct InterruptHandler InterruptHandler;
typedef struct InterruptManager InterruptManager;

// Handler function pointer type
typedef void (*InterruptHandlerFunc)(InterruptManager* manager, uint32_t vector, uint64_t error_code);

// Interrupt Handler structure
struct InterruptHandler {
    uint32_t id;
    InterruptType type;
    uint32_t vector;
    InterruptHandlerFunc handler;
    void* context;
    bool enabled;
    uint32_t priority;
    char name[32];
    struct InterruptHandler* next;
};

// Interrupt Manager with SOLID principles
struct InterruptManager {
    // IDT (Interrupt Descriptor Table)
    struct {
        uint64_t base;
        uint16_t limit;
        uint64_t entries[IDT_ENTRIES];
    } idt;
    
    // Handler management
    InterruptHandler* handlers[IDT_ENTRIES];
    InterruptHandler* handler_list;
    uint32_t next_handler_id;
    
    // Statistics
    uint64_t interrupt_count[IDT_ENTRIES];
    uint64_t total_interrupts;
    uint64_t spurious_interrupts;
    uint64_t nested_interrupts;
    
    // State
    bool interrupts_enabled;
    uint32_t current_nesting_level;
    uint32_t current_vector;
    
    // Hardware abstraction
    void (*enable_interrupts)(void);
    void (*disable_interrupts)(void);
    uint32_t (*get_irq_mask)(void);
    void (*set_irq_mask)(uint32_t mask);
    
    // Configuration
    uint32_t max_nesting_level;
    bool auto_eoi;
    uint32_t priority_levels;
};

// CPU Context structure for interrupt handling
typedef struct {
    uint64_t rax, rbx, rcx, rdx;
    uint64_t rsi, rdi, rbp, rsp;
    uint64_t r8, r9, r10, r11;
    uint64_t r12, r13, r14, r15;
    uint64_t rip, rflags;
    uint64_t cs, ss, ds, es, fs, gs;
    uint64_t error_code;
    uint32_t vector;
} CPUContext;

// IDT Entry structure
typedef struct {
    uint16_t offset_low;
    uint16_t selector;
    uint8_t ist;
    uint8_t type_attr;
    uint16_t offset_middle;
    uint32_t offset_high;
    uint32_t zero;
} __attribute__((packed)) IDTEntry;

// OOP: Default interrupt handlers
static void default_exception_handler(InterruptManager* manager, uint32_t vector, uint64_t error_code);
static void default_irq_handler(InterruptManager* manager, uint32_t vector, uint64_t error_code);
static void default_spurious_handler(InterruptManager* manager, uint32_t vector, uint64_t error_code);
static void syscall_handler(InterruptManager* manager, uint32_t vector, uint64_t error_code);

// Hardware abstraction layer implementations
static void x86_enable_interrupts(void) {
    __asm__ volatile ("sti");
}

static void x86_disable_interrupts(void) {
    __asm__ volatile ("cli");
}

static uint32_t x86_get_irq_mask(void) {
    uint32_t mask;
    __asm__ volatile ("inb $0x21, %0" : "=a"(mask));
    return mask;
}

static void x86_set_irq_mask(uint32_t mask) {
    __asm__ volatile ("outb %0, $0x21" : : "a"(mask));
}

// Interrupt Manager Constructor
InterruptManager* sigma_interrupt_manager_create(void) {
    InterruptManager* manager = (InterruptManager*)malloc(sizeof(InterruptManager));
    if (!manager) return NULL;
    
    // Initialize fields
    memset(manager, 0, sizeof(InterruptManager));
    
    manager->idt.base = (uint64_t)&manager->idt.entries;
    manager->idt.limit = sizeof(manager->idt.entries) - 1;
    manager->next_handler_id = 1;
    manager->interrupts_enabled = false;
    manager->current_nesting_level = 0;
    manager->max_nesting_level = 16;
    manager->auto_eoi = true;
    manager->priority_levels = 8;
    
    // Set hardware abstraction functions
    manager->enable_interrupts = x86_enable_interrupts;
    manager->disable_interrupts = x86_disable_interrupts;
    manager->get_irq_mask = x86_get_irq_mask;
    manager->set_irq_mask = x86_set_irq_mask;
    
    // Initialize IDT entries
    for (int i = 0; i < IDT_ENTRIES; i++) {
        manager->idt.entries[i] = 0;
    }
    
    // Set up default handlers
    sigma_interrupt_register_handler(manager, EXC_DIVIDE_BY_ZERO, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Divide by Zero");
    sigma_interrupt_register_handler(manager, EXC_DEBUG, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Debug");
    sigma_interrupt_register_handler(manager, EXC_NMI, default_exception_handler, 
                                  INTERRUPT_TYPE_NMI, "NMI");
    sigma_interrupt_register_handler(manager, EXC_BREAKPOINT, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Breakpoint");
    sigma_interrupt_register_handler(manager, EXC_OVERFLOW, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Overflow");
    sigma_interrupt_register_handler(manager, EXC_BOUND_RANGE, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Bound Range");
    sigma_interrupt_register_handler(manager, EXC_INVALID_OPCODE, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Invalid Opcode");
    sigma_interrupt_register_handler(manager, EXC_DEVICE_NOT_AVAILABLE, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Device Not Available");
    sigma_interrupt_register_handler(manager, EXC_DOUBLE_FAULT, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Double Fault");
    sigma_interrupt_register_handler(manager, EXC_INVALID_TSS, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Invalid TSS");
    sigma_interrupt_register_handler(manager, EXC_SEGMENT_NOT_PRESENT, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Segment Not Present");
    sigma_interrupt_register_handler(manager, EXC_STACK_SEGMENT_FAULT, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Stack Segment Fault");
    sigma_interrupt_register_handler(manager, EXC_GENERAL_PROTECTION, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "General Protection");
    sigma_interrupt_register_handler(manager, EXC_PAGE_FAULT, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Page Fault");
    sigma_interrupt_register_handler(manager, EXC_X87_FPU_ERROR, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "x87 FPU Error");
    sigma_interrupt_register_handler(manager, EXC_ALIGNMENT_CHECK, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Alignment Check");
    sigma_interrupt_register_handler(manager, EXC_MACHINE_CHECK, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "Machine Check");
    sigma_interrupt_register_handler(manager, EXC_SIMD_FP_EXCEPTION, default_exception_handler, 
                                  INTERRUPT_TYPE_EXCEPTION, "SIMD FP Exception");
    
    // Set up system call handler
    sigma_interrupt_register_handler(manager, SYSCALL_VECTOR, syscall_handler, 
                                  INTERRUPT_TYPE_SOFTWARE, "System Call");
    
    // Set up default IRQ handlers
    for (int i = 0; i < 16; i++) {
        char name[32];
        snprintf(name, sizeof(name), "IRQ %d", i);
        sigma_interrupt_register_handler(manager, IRQ_BASE + i, default_irq_handler, 
                                      INTERRUPT_TYPE_IRQ, name);
    }
    
    return manager;
}

// Register interrupt handler (Factory Method)
uint32_t sigma_interrupt_register_handler(InterruptManager* manager, uint32_t vector, 
                                        InterruptHandlerFunc handler, InterruptType type, 
                                        const char* name) {
    if (!manager || !handler || vector >= IDT_ENTRIES) return 0;
    
    InterruptHandler* ih = (InterruptHandler*)malloc(sizeof(InterruptHandler));
    if (!ih) return 0;
    
    ih->id = manager->next_handler_id++;
    ih->type = type;
    ih->vector = vector;
    ih->handler = handler;
    ih->context = NULL;
    ih->enabled = true;
    ih->priority = 0;
    strncpy(ih->name, name, sizeof(ih->name) - 1);
    
    // Add to handler list
    ih->next = manager->handler_list;
    manager->handler_list = ih;
    
    // Add to vector handler list
    ih->next = manager->handlers[vector];
    manager->handlers[vector] = ih;
    
    return ih->id;
}

// Unregister interrupt handler
bool sigma_interrupt_unregister_handler(InterruptManager* manager, uint32_t handler_id) {
    if (!manager || handler_id == 0) return false;
    
    // Remove from handler list
    InterruptHandler** prev = &manager->handler_list;
    while (*prev) {
        if ((*prev)->id == handler_id) {
            InterruptHandler* to_remove = *prev;
            *prev = to_remove->next;
            
            // Remove from vector handler list
            for (int i = 0; i < IDT_ENTRIES; i++) {
                InterruptHandler** vprev = &manager->handlers[i];
                while (*vprev) {
                    if (*vprev == to_remove) {
                        *vprev = to_remove->next;
                        break;
                    }
                    vprev = &(*vprev)->next;
                }
            }
            
            free(to_remove);
            return true;
        }
        prev = &(*prev)->next;
    }
    
    return false;
}

// Set IDT entry
void sigma_interrupt_set_idt_entry(InterruptManager* manager, uint32_t vector, 
                                 uint64_t handler_addr, uint8_t type_attr) {
    if (vector >= IDT_ENTRIES) return;
    
    IDTEntry* entry = (IDTEntry*)&manager->idt.entries[vector];
    
    entry->offset_low = handler_addr & 0xFFFF;
    entry->selector = 0x08; // Kernel code segment
    entry->ist = 0;
    entry->type_attr = type_attr;
    entry->offset_middle = (handler_addr >> 16) & 0xFFFF;
    entry->offset_high = (handler_addr >> 32) & 0xFFFFFFFF;
    entry->zero = 0;
}

// Load IDT
void sigma_interrupt_load_idt(InterruptManager* manager) {
    struct {
        uint16_t limit;
        uint64_t base;
    } __attribute__((packed)) idtr = {
        .limit = manager->idt.limit,
        .base = manager->idt.base
    };
    
    __asm__ volatile ("lidt %0" : : "m"(idtr));
}

// Enable/disable interrupts
void sigma_interrupt_enable(InterruptManager* manager) {
    if (manager && manager->enable_interrupts) {
        manager->enable_interrupts();
        manager->interrupts_enabled = true;
    }
}

void sigma_interrupt_disable(InterruptManager* manager) {
    if (manager && manager->disable_interrupts) {
        manager->disable_interrupts();
        manager->interrupts_enabled = false;
    }
}

// Main interrupt dispatcher
void sigma_interrupt_dispatch(InterruptManager* manager, CPUContext* context) {
    if (!manager || !context) return;
    
    uint32_t vector = context->vector;
    
    // Check for spurious interrupts
    if (vector >= IRQ_BASE) {
        uint32_t irq = vector - IRQ_BASE;
        uint32_t mask = manager->get_irq_mask();
        if (!(mask & (1 << irq))) {
            manager->spurious_interrupts++;
            return;
        }
    }
    
    // Update statistics
    manager->interrupt_count[vector]++;
    manager->total_interrupts++;
    manager->current_vector = vector;
    
    // Check nesting level
    if (manager->current_nesting_level >= manager->max_nesting_level) {
        manager->nested_interrupts++;
        return;
    }
    
    manager->current_nesting_level++;
    
    // Call all handlers for this vector
    InterruptHandler* handler = manager->handlers[vector];
    while (handler) {
        if (handler->enabled && handler->handler) {
            handler->handler(manager, vector, context->error_code);
        }
        handler = handler->next;
    }
    
    // Send EOI if auto-eoi is enabled
    if (manager->auto_eoi && vector >= IRQ_BASE) {
        uint32_t irq = vector - IRQ_BASE;
        if (irq < 8) {
            __asm__ volatile ("outb %0, $0x20" : : "a"(0x20)); // PIC EOI
        } else {
            __asm__ volatile ("outb %0, $0x20" : : "a"(0x20)); // PIC EOI
            __asm__ volatile ("outb %0, $0xA0" : : "a"(0x20)); // Slave PIC EOI
        }
    }
    
    manager->current_nesting_level--;
    manager->current_vector = 0;
}

// Default exception handler
static void default_exception_handler(InterruptManager* manager, uint32_t vector, uint64_t error_code) {
    // Log exception
    printf("Exception %d occurred, error code: 0x%lx\n", vector, error_code);
    
    // Handle specific exceptions
    switch (vector) {
        case EXC_PAGE_FAULT:
            printf("Page fault at address: 0x%lx\n", sigma_get_cr2());
            break;
        case EXC_GENERAL_PROTECTION:
            printf("General protection fault\n");
            break;
        case EXC_DIVIDE_BY_ZERO:
            printf("Division by zero\n");
            break;
        default:
            printf("Unknown exception\n");
            break;
    }
    
    // In a real OS, this would kill the current process or panic
}

// Default IRQ handler
static void default_irq_handler(InterruptManager* manager, uint32_t vector, uint64_t error_code) {
    uint32_t irq = vector - IRQ_BASE;
    printf("IRQ %d occurred\n", irq);
    
    // This would be replaced by actual device drivers
}

// Spurious interrupt handler
static void default_spurious_handler(InterruptManager* manager, uint32_t vector, uint64_t error_code) {
    manager->spurious_interrupts++;
}

// System call handler
static void syscall_handler(InterruptManager* manager, uint32_t vector, uint64_t error_code) {
    // Get system call number from register
    uint64_t syscall_number;
    __asm__ volatile ("mov %%rax, %0" : "=r"(syscall_number));
    
    // Handle system call
    switch (syscall_number) {
        case 0: // sys_exit
            // Handle exit
            break;
        case 1: // sys_write
            // Handle write
            break;
        case 2: // sys_read
            // Handle read
            break;
        default:
            printf("Unknown system call: %ld\n", syscall_number);
            break;
    }
}

// Get CR2 register (page fault address)
static uint64_t sigma_get_cr2(void) {
    uint64_t cr2;
    __asm__ volatile ("mov %%cr2, %0" : "=r"(cr2));
    return cr2;
}

// Interrupt statistics
typedef struct {
    uint32_t vector;
    char name[32];
    uint64_t count;
    InterruptType type;
} InterruptStats;

void sigma_interrupt_get_stats(InterruptManager* manager, InterruptStats* stats, size_t* count) {
    if (!manager || !stats) return;
    
    size_t index = 0;
    for (int i = 0; i < IDT_ENTRIES; i++) {
        if (manager->interrupt_count[i] > 0) {
            stats[index].vector = i;
            stats[index].count = manager->interrupt_count[i];
            
            // Get handler name
            InterruptHandler* handler = manager->handlers[i];
            if (handler) {
                strncpy(stats[index].name, handler->name, sizeof(stats[index].name) - 1);
                stats[index].type = handler->type;
            } else {
                snprintf(stats[index].name, sizeof(stats[index].name), "Unknown");
                stats[index].type = INTERRUPT_TYPE_EXCEPTION;
            }
            
            index++;
        }
    }
    
    *count = index;
}

// Interrupt Manager Destructor
void sigma_interrupt_manager_destroy(InterruptManager* manager) {
    if (!manager) return;
    
    // Free all handlers
    InterruptHandler* handler = manager->handler_list;
    while (handler) {
        InterruptHandler* next = handler->next;
        free(handler);
        handler = next;
    }
    
    free(manager);
}

// Assembly interrupt stubs (would be in separate .S file)
extern void interrupt_stub_0(void);
extern void interrupt_stub_1(void);
extern void interrupt_stub_2(void);
// ... continue for all 256 vectors

// Initialize interrupt stubs
void sigma_interrupt_init_stubs(InterruptManager* manager) {
    // Set up IDT entries for all vectors
    for (int i = 0; i < IDT_ENTRIES; i++) {
        uint64_t handler_addr = (uint64_t)interrupt_stub_0 + i * 16; // Simplified
        uint8_t type_attr = 0x8E; // Present, DPL=0, 64-bit interrupt gate
        
        if (i == SYSCALL_VECTOR) {
            type_attr = 0xEE; // Present, DPL=3, 64-bit interrupt gate
        }
        
        sigma_interrupt_set_idt_entry(manager, i, handler_addr, type_attr);
    }
}

