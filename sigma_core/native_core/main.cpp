// SigmaOS Native Main Entry Point (C++ / Assembly Bootloader)
// ==========================================================
// Bypasses python.exe startup natively. Links statically with zero external libs.
// Pure Object-Oriented bare metal implementation.

#include "types.h"
#include "SigmaKernel.hpp"
#include "LinuxAbsorber.hpp"

// We provide our own global allocator to fulfill overriding new/delete.
namespace Sigma {
namespace Core {
    MemoryAllocator GlobalAllocator;
}
}

// OS Specific _start or main entry.
// Bypasses libc's __libc_start_main or MSVC's mainCRTStartup
extern "C" {

#if defined(_WIN32)
    // Bare Windows raw entry
    #pragma comment(linker, "/ENTRY:SigmaBoot_Win32")
    #pragma comment(linker, "/SUBSYSTEM:WINDOWS")
    
    extern void __stdcall ExitProcess(unsigned int uExitCode);

    void __stdcall SigmaBoot_Win32() {
        // Construct the OS Kernel
        Sigma::Core::Kernel* os = new Sigma::Core::Kernel();
        
        Sigma::Core::AutomationSubsystem* auto_sys = new Sigma::Core::AutomationSubsystem();
        Sigma::Core::SecuritySubsystem* sec_sys = new Sigma::Core::SecuritySubsystem();
        
        os->RegisterSubsystem(auto_sys);
        os->RegisterSubsystem(sec_sys);
        
        bool boot_success = os->Boot();
        
        // Endless loop for bare metal simulation or automation routine polling
        if (boot_success) {
            auto_sys->ExecuteRoutine(Sigma::Core::String("System_Personalisation_Ready"));
        }
        
        os->Shutdown();
        delete os;
        
        ExitProcess(0);
    }

#else
    // Bare Linux raw entry (no libc)
    void _start() {
        Sigma::Core::Kernel* os = new Sigma::Core::Kernel();
        
        Sigma::Core::AutomationSubsystem* auto_sys = new Sigma::Core::AutomationSubsystem();
        Sigma::Core::SecuritySubsystem* sec_sys = new Sigma::Core::SecuritySubsystem();
        
        os->RegisterSubsystem(auto_sys);
        os->RegisterSubsystem(sec_sys);
        
        bool boot_success = os->Boot();
        
        if (boot_success) {
            auto_sys->ExecuteRoutine(Sigma::Core::String("System_Personalisation_Ready"));
        }
        
        os->Shutdown();
        delete os;
        
        // syscall exit
        asm volatile(
            "mov $60, %rax\n" // sys_exit
            "xor %rdi, %rdi\n"
            "syscall"
        );
    }
#endif

} // extern "C"
