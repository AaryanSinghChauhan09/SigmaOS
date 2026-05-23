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
    while (l->src[l->pos] == ' ' || l->src[l->pos] == '\n' || l->src[l->pos] == '\t' || l->src[l->pos] == '\r')
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
        while (is_alpha(l->src[l->pos]) || is_digit(l->src[l->pos])) {
            if (j < 63) l->tok_str[j++] = l->src[l->pos];
            l->pos++;
        }
        l->tok_str[j] = '\0';

        if (l->tok_str[0]=='i' && l->tok_str[1]=='n' && l->tok_str[2]=='t' && l->tok_str[3]=='\0')
            l->tok_type = TOK_KW_INT;
        else if (l->tok_str[0]=='r' && l->tok_str[1]=='e' && l->tok_str[2]=='t' && l->tok_str[3]=='u' && l->tok_str[4]=='r' && l->tok_str[5]=='n' && l->tok_str[6]=='\0')
            l->tok_type = TOK_KW_RET;
        else if (l->tok_str[0]=='i' && l->tok_str[1]=='f' && l->tok_str[2]=='\0')
            l->tok_type = TOK_KW_IF;
        else if (l->tok_str[0]=='w' && l->tok_str[1]=='h' && l->tok_str[2]=='i' && l->tok_str[3]=='l' && l->tok_str[4]=='e' && l->tok_str[5]=='\0')
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

/* ─── AST Node Definitions ─── */
enum ASTNodeType {
    AST_PROGRAM,
    AST_FUNC_DECL,
    AST_VAR_DECL,
    AST_BLOCK,
    AST_RETURN,
    AST_IF,
    AST_WHILE,
    AST_ASSIGN,
    AST_BINOP,
    AST_LITERAL,
    AST_IDENTIFIER
};

struct ASTNode {
    ASTNodeType type;
    int int_val;
    char str_val[64];
    u32 op; /* For binops */
    ASTNode* left;
    ASTNode* right;
    ASTNode* body;
    ASTNode* next; /* For lists of statements */
};

/* Very rudimentary bump allocator for AST nodes */
static char ast_arena[65536];
static u32 ast_arena_pos = 0;

static ASTNode* alloc_node(ASTNodeType type) {
    if (ast_arena_pos + sizeof(ASTNode) > sizeof(ast_arena)) return nullptr;
    ASTNode* node = (ASTNode*)(ast_arena + ast_arena_pos);
    ast_arena_pos += sizeof(ASTNode);
    node->type = type;
    node->left = node->right = node->body = node->next = nullptr;
    return node;
}

/* ─── Parser Stub ─── */
static void expect(Lexer* l, u32 type) {
    if (l->tok_type == type) {
        lexer_advance(l);
    } else {
        sigma_vga_printf("[CC] Parse error: expected %d, got %d\n", type, l->tok_type);
    }
}

static ASTNode* parse_expression(Lexer* l) {
    if (l->tok_type == TOK_INT) {
        ASTNode* n = alloc_node(AST_LITERAL);
        n->int_val = l->tok_int;
        lexer_advance(l);
        return n;
    } else if (l->tok_type == TOK_IDENT) {
        ASTNode* n = alloc_node(AST_IDENTIFIER);
        u32 i = 0; while (l->tok_str[i]) { n->str_val[i] = l->tok_str[i]; i++; } n->str_val[i] = '\0';
        lexer_advance(l);
        
        if (l->tok_type == TOK_ASSIGN) {
            lexer_advance(l);
            ASTNode* assign = alloc_node(AST_ASSIGN);
            assign->left = n;
            assign->right = parse_expression(l);
            return assign;
        }
        return n;
    }
    sigma_vga_printf("[CC] Parse error: expected expression\n");
    return nullptr;
}

static ASTNode* parse_statement(Lexer* l) {
    if (l->tok_type == TOK_KW_RET) {
        lexer_advance(l);
        ASTNode* n = alloc_node(AST_RETURN);
        n->left = parse_expression(l);
        expect(l, TOK_SEMI);
        return n;
    } else if (l->tok_type == TOK_KW_INT) {
        lexer_advance(l);
        ASTNode* n = alloc_node(AST_VAR_DECL);
        u32 i = 0; while (l->tok_str[i]) { n->str_val[i] = l->tok_str[i]; i++; } n->str_val[i] = '\0';
        expect(l, TOK_IDENT);
        expect(l, TOK_SEMI);
        return n;
    }
    
    ASTNode* expr = parse_expression(l);
    expect(l, TOK_SEMI);
    return expr;
}

static ASTNode* parse_function(Lexer* l) {
    expect(l, TOK_KW_INT); /* Return type */
    ASTNode* func = alloc_node(AST_FUNC_DECL);
    u32 i = 0; while (l->tok_str[i]) { func->str_val[i] = l->tok_str[i]; i++; } func->str_val[i] = '\0';
    expect(l, TOK_IDENT);
    expect(l, TOK_LPAREN);
    expect(l, TOK_RPAREN);
    expect(l, TOK_LBRACE);
    
    ASTNode* block = alloc_node(AST_BLOCK);
    ASTNode* current = nullptr;
    
    while (l->tok_type != TOK_RBRACE && l->tok_type != TOK_EOF) {
        ASTNode* stmt = parse_statement(l);
        if (!block->body) {
            block->body = stmt;
            current = stmt;
        } else {
            current->next = stmt;
            current = stmt;
        }
    }
    expect(l, TOK_RBRACE);
    func->body = block;
    return func;
}

static ASTNode* parse_program(Lexer* l) {
    ASTNode* prog = alloc_node(AST_PROGRAM);
    ASTNode* current = nullptr;
    while (l->tok_type != TOK_EOF) {
        ASTNode* func = parse_function(l);
        if (!prog->body) {
            prog->body = func;
            current = func;
        } else {
            current->next = func;
            current = func;
        }
    }
    return prog;
}

/* ─── Code Generation ─── */
static void generate_code(ASTNode* node) {
    if (!node) return;
    switch (node->type) {
        case AST_PROGRAM:
            generate_code(node->body);
            break;
        case AST_FUNC_DECL:
            sigma_vga_printf("global %s\n", node->str_val);
            sigma_vga_printf("%s:\n", node->str_val);
            sigma_vga_printf("  push rbp\n");
            sigma_vga_printf("  mov rbp, rsp\n");
            generate_code(node->body);
            sigma_vga_printf("  pop rbp\n");
            sigma_vga_printf("  ret\n");
            generate_code(node->next);
            break;
        case AST_BLOCK:
            generate_code(node->body);
            break;
        case AST_RETURN:
            generate_code(node->left);
            break;
        case AST_LITERAL:
            sigma_vga_printf("  mov rax, %d\n", node->int_val);
            break;
        case AST_IDENTIFIER:
            /* Very naive local var lookup placeholder */
            sigma_vga_printf("  mov rax, [rbp-8] ; (naive read %s)\n", node->str_val);
            break;
        case AST_ASSIGN:
            generate_code(node->right);
            sigma_vga_printf("  mov [rbp-8], rax ; (naive store to %s)\n", node->left->str_val);
            break;
        case AST_VAR_DECL:
            sigma_vga_printf("  ; var %s\n", node->str_val);
            break;
        default:
            break;
    }
    if (node->type != AST_PROGRAM && node->type != AST_FUNC_DECL) {
        generate_code(node->next);
    }
}

extern "C" int sigma_cc_main(int argc, char** argv) {
    sigma_vga_printf("SigmaCC v0.2 [Sovereign C-Subset Compiler]\n");
    if (argc < 2) {
        sigma_vga_printf("Usage: cc <source.c>\n");
        return 1;
    }
    sigma_vga_printf("Compiling %s...\n", argv[1]);
    
    /* Mock source code for testing if actual file reading is not available */
    const char* source = "int main() { int x; x = 42; return x; }";
    
    Lexer l;
    l.src = source;
    l.pos = 0;
    lexer_advance(&l);
    
    ASTNode* ast = parse_program(&l);
    sigma_vga_printf("[CC] Parse complete. Generating assembly...\n");
    
    generate_code(ast);
    
    sigma_vga_printf("[CC] Done.\n");
    return 0;
}
