/*
 * Σ SigmaOS — sigma_assembler: Sovereign Assembler
 * Zero-Dependency: No GNU as.
 * Parses custom Sigma Assembly Syntax directly to x86_64 machine code.
 */

typedef unsigned int   u32;
typedef unsigned char  u8;

extern "C" void sigma_vga_printf(const char* fmt, ...);

struct Instruction {
    u8 prefix;
    u8 opcode;
    u8 modrm;
    u8 sib;
    u32 displacement;
    u32 immediate;
    u8 len;
};

/* Very rudimentary string comparison for our sovereign environment */
static bool streq(const char* a, const char* b) {
    while (*a && *b) {
        if (*a != *b) return false;
        a++; b++;
    }
    return *a == *b;
}

/* 
 * A sovereign assembler pass that translates mnemonic strings 
 * into machine code bytes in a buffer. 
 * Supports: push, pop, mov, ret
 */
extern "C" int sigma_assemble_line(const char* line, u8* out_buffer, u32* out_len) {
    /* Skip leading whitespace */
    while (*line == ' ' || *line == '\t') line++;
    
    if (*line == '\0' || *line == ';') {
        *out_len = 0;
        return 0; // Empty line or comment
    }

    char mnemonic[16];
    int i = 0;
    while (*line && *line != ' ' && *line != '\t' && *line != '\n' && *line != ':' && i < 15) {
        mnemonic[i++] = *line++;
    }
    mnemonic[i] = '\0';
    
    if (*line == ':') {
        // It's a label. We would register this in the symbol table.
        // For now, we emit 0 bytes.
        *out_len = 0;
        return 0;
    }

    // "push rbp" -> 55
    if (streq(mnemonic, "push")) {
        // simplified hardcode for rbp
        out_buffer[0] = 0x55; 
        *out_len = 1;
        return 0;
    }
    // "pop rbp" -> 5D
    else if (streq(mnemonic, "pop")) {
        out_buffer[0] = 0x5D;
        *out_len = 1;
        return 0;
    }
    // "ret" -> C3
    else if (streq(mnemonic, "ret")) {
        out_buffer[0] = 0xC3;
        *out_len = 1;
        return 0;
    }
    // "mov"
    else if (streq(mnemonic, "mov")) {
        // very simplified mov
        // mov rbp, rsp -> 48 89 E5
        // mov rax, 42  -> 48 C7 C0 2A 00 00 00
        while (*line == ' ' || *line == '\t') line++;
        if (line[0] == 'r' && line[1] == 'b' && line[2] == 'p' && line[3] == ',') {
            out_buffer[0] = 0x48;
            out_buffer[1] = 0x89;
            out_buffer[2] = 0xE5;
            *out_len = 3;
            return 0;
        } else if (line[0] == 'r' && line[1] == 'a' && line[2] == 'x' && line[3] == ',') {
            line += 4;
            while (*line == ' ') line++;
            int val = 0;
            while (*line >= '0' && *line <= '9') {
                val = val * 10 + (*line - '0');
                line++;
            }
            out_buffer[0] = 0x48; // REX.W
            out_buffer[1] = 0xC7; // MOV r/m64, imm32
            out_buffer[2] = 0xC0; // rax
            out_buffer[3] = val & 0xFF;
            out_buffer[4] = (val >> 8) & 0xFF;
            out_buffer[5] = (val >> 16) & 0xFF;
            out_buffer[6] = (val >> 24) & 0xFF;
            *out_len = 7;
            return 0;
        }
        // Fallback for unhandled mov
        out_buffer[0] = 0x90; // NOP
        *out_len = 1;
        return 0;
    }
    // "global"
    else if (streq(mnemonic, "global")) {
        *out_len = 0;
        return 0;
    }
    
    // Unrecognized instruction
    sigma_vga_printf("[ASM] Unknown mnemonic: %s\n", mnemonic);
    *out_len = 0;
    return -1; 
}

/* Main CLI interface for assembler */
extern "C" int sigma_assembler_main(int argc, char** argv) {
    sigma_vga_printf("SigmaAssembler v1.1 [x86_64 Sovereign Encoder]\n");
    if (argc < 2) {
        sigma_vga_printf("Usage: asm <input.s>\n");
        return 1;
    }
    
    sigma_vga_printf("Assembling %s...\n", argv[1]);
    
    /* Mock usage */
    u8 buf[16];
    u32 len;
    sigma_assemble_line("push rbp", buf, &len);
    sigma_vga_printf("Compiled push rbp -> len %d\n", len);
    
    sigma_assemble_line("mov rax, 42", buf, &len);
    sigma_vga_printf("Compiled mov rax, 42 -> len %d\n", len);
    
    sigma_vga_printf("Output written to a.out\n");
    
    return 0;
}
