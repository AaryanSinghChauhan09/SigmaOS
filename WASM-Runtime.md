# WASM-Runtime



The SigmaOS WASM Runtime provides a high-performance, sandboxed execution environment for userland applications and isolated kernel tasks. Leveraging the **Sovereign PSE (Portable Shard Execution)** model, WASM modules can run natively on the silicon bus without traditional instruction set translation overhead.


graph LR
    Shard[WASM Shard] --> Engine[SovereignWasmEngine]
    Engine --> LinearMem[Isolated Linear Memory]
    Engine --> Syscalls[Sovereign Syscall Bridge]
    LinearMem --> PMM[SovereignPMM]






The runtime is implemented as a modular C++ singleton:


class SovereignWasmEngine {
public:
    static SovereignWasmEngine& getInstance();
    void loadShard(const uint8_t* bytecode, size_t size);
    void execute();
private:
    // ...
};




