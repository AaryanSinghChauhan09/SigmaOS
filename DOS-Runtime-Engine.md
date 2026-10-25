# DOS and Retro Computing Runtime Engine

> **Principles:** ABSTRACTION · CLEAR ABSTRACTION · COMPATIBILITY · DIRECT DEVICE CONTROL · INTERFACE SEGREGATION · LOW LEVEL CONTROL · MANUAL MEMORY CONTROL · MEMORY DISCIPLINE

This specification establishes the retro computing execution subsystem, real-mode virtual machine, DOS interrupt dispatcher, DPMI protected-mode host, and BASIC language runtime in **SigmaOS**.

---

## 1. Subsystem Architecture

SigmaOS provides a clean-room, safe Rust implementation of retro 16-bit and 32-bit computing runtimes, enabling legacy software execution alongside modern multi-core microkernel tasks:

```mermaid
graph TD
    subgraph Userspace Applications
        DOS_APP["16-Bit DOS Applications (.COM / .EXE)"]
        DPMI_APP["32-Bit Protected Mode Binaries (DOS/4GW / DJGPP)"]
        BASIC_APP["BASIC Source Programs (GW-BASIC / QBASIC)"]
        WIN16_APP["Win16 / WOW32 Binaries"]
    end

    subgraph DOS Subsystem (src/kernel/dos)
        INT21["INT 21h Dispatcher (File I/O, Mem, Terminate)"]
        DPMI["DPMI 0.9+ Server (LDT Allocation, Mode Switching)"]
        NTVDM["NTVDM Session Bridge (Console Buffer, 16->32 Thunks)"]
        BASIC["BASIC Interpreter (AST, Eval, Lexer, Control Flow)"]
    end

    subgraph Hardware Virtualization
        V86["Virtual 8086 Machine (Registers, Flags, IVT, Port Trapping)"]
    end

    DOS_APP --> INT21
    DOS_APP --> V86
    DPMI_APP --> DPMI
    BASIC_APP --> BASIC
    WIN16_APP --> NTVDM
    INT21 --> V86
    DPMI --> V86
```

---

## 2. Core Components

### 2.1 INT 21h Dispatcher (`int21h.rs`)
- **Service Categories:** Clean-room implementation of MS-DOS / PC-DOS API functions via AH register multiplexing.
- **Console & Character I/O:** `01h` (GetChar), `02h` (PrintChar), `09h` (PrintString `$`-terminated).
- **File System Operations:** `3Ch` (Create), `3Dh` (Open), `3Eh` (Close), `3Fh` (Read), `40h` (Write), `41h` (Delete), `56h` (Rename).
- **Memory Management:** `48h` (AllocMem in paragraphs), `49h` (FreeMem), `4Ah` (ModifyMem).
- **Process Lifecycle:** `4Ch` (Terminate with return code).

### 2.2 DPMI 0.9+ Protected-Mode Server (`dpmi.rs`)
- **Mode Switching:** INT 2Fh AX=1687h entry point for switching from real-mode or V86 to 16/32-bit protected mode.
- **Descriptor Table Management:** Dynamic Local Descriptor Table (LDT) segment allocation, base/limit programming, and access rights configuration.
- **Linear Memory Management:** INT 31h AX=0501h linear address block mapping, physical locking, and page-aligned allocation.

### 2.3 Virtual 8086 Mode Machine (`vdm.rs`)
- **Register State:** Full 8086 register file (`ax`, `bx`, `cx`, `dx`, `si`, `di`, `bp`, `sp`, `cs`, `ds`, `es`, `ss`, `ip`, `flags`).
- **Interrupt Vector Table (IVT):** 256 4-byte vectors mapped to `0000:0000h` with automated stack push/pop on software interrupts (`INT n` / `IRET`).
- **I/O Port Trapping:** Configurable port trap dispatchers for legacy peripherals (PS/2 keyboard, timer 8253/8254, PIC 8259A, VGA registers).

### 2.4 NT Virtual DOS Machine Bridge (`ntvdm.rs`)
- **Memory Configuration:** Conventional 640KB RAM, Extended Memory Specification (XMS), and Expanded Memory Specification (EMS).
- **Thunking Mechanism:** Generic 16-to-32 bit thunk tables (`Ntvdm16BitThunk`) enabling legacy 16-bit applications to call SigmaOS 64-bit kernel APIs.
- **Console Terminal:** Dual-mode text (80x25 / 40x25) and graphics (CGA / EGA / VGA) frame buffer simulation.

### 2.5 BASIC Language Engine (`basic_engine.rs`)
- **Dialect Compatibility:** Clean-room interpreter supporting classic Dartmouth, Microsoft GW-BASIC, and QBASIC command syntax.
- **Statement Support:** `PRINT`, `INPUT`, `LET`, `FOR ... TO ... STEP ... NEXT`, `GOTO`, `GOSUB ... RETURN`, `IF ... THEN ... ELSE`, `DIM`, `REM`.
- **Expression Evaluator:** Polymorphic arithmetic and logical operators supporting integer, floating-point, and string values.

---

## 3. Directory Structure

```
src/kernel/dos/
├── mod.rs              # Re-exports and submodule wiring
├── int21h.rs           # Clean-room INT 21h DOS service dispatcher
├── dpmi.rs             # DPMI 0.9+ protected mode server
├── vdm.rs              # Virtual 8086 mode processor emulator
├── ntvdm.rs            # NT Virtual DOS Machine console & thunks
└── basic_engine.rs     # BASIC AST parser and runtime engine
```
