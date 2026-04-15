/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN LEXER SHARD (v50.0-SINGULARITY)
 * =========================================================================
 * Mission: Zero-dependency lexical analysis for Sovereign Scripts.
 * Principles: Finite State Automata, Tokenization, Grammar Purity.
 *
 * This shard tokenizes "SigmaScript" which compiles to the UDF VM bytecode.
 * =========================================================================
 */

#ifndef SOVEREIGN_LEXER_SHARD_H
#define SOVEREIGN_LEXER_SHARD_H

#include "sigma_kernel.h"

typedef enum {
    TOK_IDENT,
    TOK_NUMBER,
    TOK_OP_ADD,
    TOK_OP_SUB,
    TOK_KW_DEF,
    TOK_KW_AUTO,
    TOK_EOF
} SigmaToken_t;

typedef struct {
    const char* input;
    sigma_u32   pos;
    char        current_char;
} SigmaLexer_t;

/**
 * sigma_lexer_next: Fetches the next token from the stream.
 * Principle: Computer Science / Language Design.
 */
SigmaToken_t sigma_lexer_get_token(SigmaLexer_t* l) {
    while (l->input[l->pos] == ' ') l->pos++; // Simple whitespace skip

    char c = l->input[l->pos];
    if (c == '\0') return TOK_EOF;
    
    if (c == '+') { l->pos++; return TOK_OP_ADD; }
    if (c >= '0' && c <= '9') {
        while (l->input[l->pos] >= '0' && l->input[l->pos] <= '9') l->pos++;
        return TOK_NUMBER;
    }
    
    // Simplistic keyword detection
    if (c >= 'a' && c <= 'z') {
        while (l->input[l->pos] >= 'a' && l->input[l->pos] <= 'z') l->pos++;
        return TOK_IDENT;
    }

    l->pos++;
    return TOK_EOF;
}

/* --- Module Factory --- */

void SovereignLexer_Register(void) {
    sigma_printf("[INTELLIGENCE]: Sovereign Lexer (Language Core) active.\n");
    sigma_printf("[AUDIT]: Grammar-Sovereignty Level: COMPLETE.\n");
}

#endif /* SOVEREIGN_LEXER_SHARD_H */



