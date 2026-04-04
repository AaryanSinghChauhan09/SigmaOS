/*
 * =========================================================================
 * Σ SIGMAOS AST INDEXER: LOCAL CODEBASE UNDERSTANDING ENGINE
 * =========================================================================
 * Mission: Ultra-Fast Native Indexing for Agentic Context Retrieval.
 * Design: No External Parsers / Pure C11 / String Masking / Shard Index.
 * =========================================================================
 */

#include "../SigmaC11.h"

#define MAX_SYMBOLS 1024
#define MAX_PATH 256

typedef enum {
    SYM_FUNCTION,
    SYM_STRUCT,
    SYM_GLOBAL
} SymbolType;

typedef struct {
    char name[64];
    SymbolType type;
    int line;
    char path[MAX_PATH];
} CodeSymbol;

static CodeSymbol g_SymbolTable[MAX_SYMBOLS];
static int g_SymbolCount = 0;

/**
 * Σ Index File
 * Logic: Simple pattern matching for core C symbols.
 */
void SigmaIndexFile(const char* path, const char* content) {
    char line[512];
    int line_num = 1;
    const char* ptr = content;
    
    while (*ptr && g_SymbolCount < MAX_SYMBOLS) {
        // Extract line
        int i = 0;
        while (*ptr && *ptr != '\n' && i < 511) {
            line[i++] = *ptr++;
        }
        line[i] = '\0';
        if (*ptr == '\n') ptr++;
        
        // Simple Function Match: type name(args) {
        if ((sigma_strstr(line, "void ") || sigma_strstr(line, "int ") || sigma_strstr(line, "char* ")) && 
            sigma_strrchr(line, '(') && sigma_strrchr(line, ')') && !sigma_strrchr(line, ';')) {
            
            CodeSymbol* sym = &g_SymbolTable[g_SymbolCount++];
            sym->type = SYM_FUNCTION;
            sym->line = line_num;
            sigma_strncpy(sym->path, path, MAX_PATH - 1);
            sym->path[MAX_PATH - 1] = '\0';
            
            // Extract Name (Basic)
            const char* name_start = sigma_strstr(line, " ") + 1;
            const char* name_end = sigma_strstr(name_start, "(");
            if (name_end) {
                sigma_size_t len = (sigma_size_t)(name_end - name_start);
                if (len > 63) len = 63;
                sigma_memcpy(sym->name, name_start, len);
                sym->name[len] = '\0';
            }
        }
        
        // Simple Struct Match: struct name {
        if (sigma_strstr(line, "struct ") && sigma_strrchr(line, '{')) {
            CodeSymbol* sym = &g_SymbolTable[g_SymbolCount++];
            sym->type = SYM_STRUCT;
            sym->line = line_num;
            sigma_strncpy(sym->path, path, MAX_PATH - 1);
            sym->path[MAX_PATH - 1] = '\0';
            
            const char* name_start = sigma_strstr(line, "struct ") + 7;
            const char* name_end = sigma_strstr(name_start, " ");
            if (!name_end) name_end = sigma_strrchr(name_start, '{');
            if (name_end) {
                sigma_size_t len = (sigma_size_t)(name_end - name_start);
                if (len > 63) len = 63;
                sigma_memcpy(sym->name, name_start, len);
                sym->name[len] = '\0';
            }
        }
        
        line_num++;
    }
}

/**
 * Σ Symbol Search
 */
CodeSymbol* SigmaFindSymbol(const char* name) {
    for (int i = 0; i < g_SymbolCount; i++) {
        if (sigma_streq(g_SymbolTable[i].name, name)) {
            return &g_SymbolTable[i];
        }
    }
    return SIGMA_NULL;
}

