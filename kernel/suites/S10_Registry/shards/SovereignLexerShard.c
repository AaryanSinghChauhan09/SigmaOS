/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN LEXER ENGINE (v1.0)
 * =========================================================================
 * Mission: Tokenization of UDF expressions for the Bytecode VM.
 * Principles: Finite State Automata (FSA), Lexical Analysis.
 *
 * Implements a real Lexer for the UDF Compiler pipeline.
 * =========================================================================
 */

#include "sigma_kernel.h"

typedef enum {
    TOK_IDENT,
    TOK_NUMBER,
    TOK_OP_ADD,
    TOK_EOF
} SigmaToken_Type;

/**
 * sigma_compiler_lex: Extracts the next token from the input stream.
 */
SigmaToken_Type sigma_compiler_lex(const char* input, int* pos) {
    char c = input[(*pos)++];
    
    if (c == '\0') return TOK_EOF;
    if (c == '+')  return TOK_OP_ADD;
    if (c >= '0' && c <= '9') return TOK_NUMBER;
    
    return TOK_IDENT;
}

/* --- Module Factory --- */

void SovereignLexer_Register(void) {
    sigma_printf("[ORCHESTRATION]: Sovereign Lexer Engine seeded.\n");
}



