/*
 * =========================================================================
 * Σ SIGMAOS AST INDEXER: LOCAL CODEBASE UNDERSTANDING ENGINE
 * =========================================================================
 * Mission: Ultra-Fast Native Indexing for Agentic Context Retrieval.
 * Design: No External Parsers / Pure C11 / String Masking / Shard Index.
 * =========================================================================
 */

#include <stdio.h>
#include <string.h>
#include <stdbool.h>

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
        while (*ptr && *ptr != '\n' && i < 511) line[i++] = *ptr++;
        line[i] = '\0';
        if (*ptr == '\n') ptr++;
        
        // Simple Function Match: type name(args) {
        if ((strstr(line, "void ") || strstr(line, "int ") || strstr(line, "char* ")) && 
            strchr(line, '(') && strchr(line, ')') && !strchr(line, ';')) {
            
            CodeSymbol* sym = &g_SymbolTable[g_SymbolCount++];
            sym->type = SYM_FUNCTION;
            sym->line = line_num;
            strcpy(sym->path, path);
            
            // Extract Name (Basic)
            char* name_start = strchr(line, ' ') + 1;
            char* name_end = strchr(name_start, '(');
            int len = name_end - name_start;
            strncpy(sym->name, name_start, len);
            sym->name[len] = '\0';
        }
        
        // Simple Struct Match: struct name {
        if (strstr(line, "struct ") && strchr(line, '{')) {
            CodeSymbol* sym = &g_SymbolTable[g_SymbolCount++];
            sym->type = SYM_STRUCT;
            sym->line = line_num;
            strcpy(sym->path, path);
            
            char* name_start = strstr(line, "struct ") + 7;
            char* name_end = strchr(name_start, ' ');
            if (!name_end) name_end = strchr(name_start, '{');
            int len = name_end - name_start;
            strncpy(sym->name, name_start, len);
            sym->name[len] = '\0';
        }
        
        line_num++;
    }
}

/**
 * Σ Symbol Search
 */
CodeSymbol* SigmaFindSymbol(const char* name) {
    for (int i = 0; i < g_SymbolCount; i++) {
        if (strcmp(g_SymbolTable[i].name, name) == 0) return &g_SymbolTable[i];
    }
    return NULL;
}
