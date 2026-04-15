#ifndef SIGMA_DEBUGGER_H
#define SIGMA_DEBUGGER_H

// SigmaOS System Debugging Tool
// Absorbing paradigms from gdb/lldb but streamlined for Sovereign Shards
#include "sigma_types.h"

void debug_attach_to_process(uint64_t pid);
void debug_dump_memory(void* start, uint32_t length);
void debug_trace_shard_execution(const char* shard_name);

#endif // SIGMA_DEBUGGER_H

