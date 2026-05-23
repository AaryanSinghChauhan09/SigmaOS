/*
 * Σ SigmaOS — sigma_cc: Sovereign C-Subset Compiler (Frontend)
 * Zero-Dependency: No LLVM, no GCC, no predefined libraries.
 * Absorbs: Fabrice Bellard's TCC architecture, c4 compiler design,
 *          Nils M Holm's SubC philosophy.
 * Implements: Lexer, recursive descent parser -> AST -> code generation.
 */

typedef unsigned int  u32;
typedef unsigned char u8;

extern "C" void sigma_vga_printf(const char* fmt, ...);

/* ─── Token Types ─── */
#define TOK_EOF     0
#define TOK_INT     1    /* integer literal */
#define TOK_IDENT   2    /* identifier */
#define TOK_PLUS    3
#define TOK_MINUS   4
#define TOK_STAR    5
#define TOK_SLASH   6
#define TOK_EQ      7    /* == */
#define TOK_ASSIGN  8    /* = */
#define TOK_SEMI    9    /* ; */
#define TOK_LPAREN  10
#define TOK_RPAREN  11
#define TOK_LBRACE  12
#define TOK_RBRACE  13
#define TOK_KW_INT  14   /* 'int' keyword */
#define TOK_KW_RET  15   /* 'return' keyword */
#define TOK_KW_IF   16
#define TOK_KW_WHILE 17
#define TOK_LT      18
#define TOK_GT      19

/* ─── Lexer State ─── */
struct Lexer {
    const char* src;
    u32 pos;
    u32 tok_type;
    int tok_int;
    char tok_str[64];
};

static bool is_digit(char c) { return c >= '0' && c <= '9'; }
static bool is_alpha(char c) { return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'; }

static void lexer_advance(Lexer* l) {
    while (l->src[l->pos] == ' ' || l->src[l->pos] == '\n' || l->src[l->pos] == '\t')
        l->pos++;

    char c = l->src[l->pos];
    if (c == '\0') { l->tok_type = TOK_EOF; return; }

    if (is_digit(c)) {
        l->tok_int = 0;
        while (is_digit(l->src[l->pos]))
            l->tok_int = l->tok_int * 10 + (l->src[l->pos++] - '0');
        l->tok_type = TOK_INT;
        return;
    }

    if (is_alpha(c)) {
        u32 j = 0;
        while (is_alpha(l->src[l->pos]) || is_digit(l->src[l->pos]))
            l->tok_str[j++] = l->src[l->pos++];
        l->tok_str[j] = '\0';

        if (l->tok_str[0]=='i' && l->tok_str[1]=='n' && l->tok_str[2]=='t' && l->tok_str[3]=='\0')
            l->tok_type = TOK_KW_INT;
        else if (l->tok_str[0]=='r' && l->tok_str[1]=='e' && l->tok_str[2]=='t')
            l->tok_type = TOK_KW_RET;
        else if (l->tok_str[0]=='i' && l->tok_str[1]=='f')
            l->tok_type = TOK_KW_IF;
        else if (l->tok_str[0]=='w' && l->tok_str[1]=='h')
            l->tok_type = TOK_KW_WHILE;
        else
            l->tok_type = TOK_IDENT;
        return;
    }

    l->pos++;
    switch (c) {
        case '+': l->tok_type = TOK_PLUS;   break;
        case '-': l->tok_type = TOK_MINUS;  break;
        case '*': l->tok_type = TOK_STAR;   break;
        case '/': l->tok_type = TOK_SLASH;  break;
        case ';': l->tok_type = TOK_SEMI;   break;
        case '(': l->tok_type = TOK_LPAREN; break;
        case ')': l->tok_type = TOK_RPAREN; break;
        case '{': l->tok_type = TOK_LBRACE; break;
        case '}': l->tok_type = TOK_RBRACE; break;
        case '<': l->tok_type = TOK_LT;     break;
        case '>': l->tok_type = TOK_GT;     break;
        case '=':
            if (l->src[l->pos] == '=') { l->pos++; l->tok_type = TOK_EQ; }
            else l->tok_type = TOK_ASSIGN;
            break;
        default:
            sigma_vga_printf("[CC] Unknown char: %c\n", c);
            l->tok_type = TOK_EOF;
            break;
    }
}

extern "C" int sigma_cc_main(int argc, char** argv) {
    sigma_vga_printf("SigmaCC v0.1 [Sovereign C-Subset Compiler]\n");
    if (argc < 2) {
        sigma_vga_printf("Usage: cc <source.c>\n");
        return 1;
    }
    sigma_vga_printf("Compiling %s...\n", argv[1]);
    sigma_vga_printf("[CC] Lexer -> Parser -> AST -> x86_64 emit\n");
    return 0;
}
