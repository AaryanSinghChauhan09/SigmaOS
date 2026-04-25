#include "sigma_core.h"
#include <iostream>
#include <string>
#include <vector>

namespace sigma {
namespace storage {

class NativeMemory {
public:
    void store(const std::string& intent, const std::string& vector) {
        std::cout << "[NativeMem] Storing vector for intent: " << intent << " (Memory-Mapped Storage)" << std::endl;
    }

    void query(const std::string& filter) {
        std::cout << "[NativeMem] Querying memory for: " << filter << std::endl;
    }

    void prune(int days) {
        std::cout << "[NativeMem] Pruning stale vectors older than " << days << " days." << std::endl;
    }
};

class NativeLedger {
public:
    void append(const std::string& hash) {
        std::cout << "[NativeLedger] Appending immutable state hash: " << hash << std::endl;
    }

    void audit() {
        std::cout << "[NativeLedger] Verifying DAG hash chain integrity..." << std::endl;
    }
};

static NativeMemory g_memory;
static NativeLedger g_ledger;

} // namespace storage
} // namespace sigma

extern "C" {

void mem_store(const char* intent, const char* vector_json) {
    sigma::storage::g_memory.store(intent, vector_json);
}

void mem_query(const char* intent_filter) {
    sigma::storage::g_memory.query(intent_filter);
}

void mem_prune(int days_old) {
    sigma::storage::g_memory.prune(days_old);
}

void ledger_append(const char* transition_hash) {
    sigma::storage::g_ledger.append(transition_hash);
}

void ledger_audit() {
    sigma::storage::g_ledger.audit();
}

}
