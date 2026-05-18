/**
 * sigma_codec.cpp — SovereignCodec Implementation
 * SigmaOS Zenith v15.1
 * Maps to: Syllabus-FCIT Unit I (Number Systems, Binary Arithmetic, Codes)
 */
#include "sigma_codec.h"

namespace {
    static const char HEX_UPPER[] = "0123456789ABCDEF";
    static void reverse_str(char* s, sigma_usize len) {
        for (sigma_usize i = 0, j = len - 1; i < j; i++, j--) {
            char tmp = s[i]; s[i] = s[j]; s[j] = tmp;
        }
    }
}

namespace Sigma::Codec {

// --- Number conversions ---
void decimal_to_binary(sigma_u64 val, char* out, sigma_usize out_len) {
    if (!val) { out[0]='0'; out[1]='\0'; return; }
    sigma_usize i = 0;
    while (val && i < out_len - 1) { out[i++] = (char)('0' + (val & 1)); val >>= 1; }
    out[i] = '\0'; reverse_str(out, i);
}
void decimal_to_octal(sigma_u64 val, char* out, sigma_usize out_len) {
    if (!val) { out[0]='0'; out[1]='\0'; return; }
    sigma_usize i = 0;
    while (val && i < out_len - 1) { out[i++] = (char)('0' + (val % 8)); val /= 8; }
    out[i] = '\0'; reverse_str(out, i);
}
void decimal_to_hex(sigma_u64 val, char* out, sigma_usize out_len, bool uppercase) {
    if (!val) { out[0]='0'; out[1]='\0'; return; }
    sigma_usize i = 0;
    while (val && i < out_len - 1) { out[i++] = HEX_UPPER[val & 0xF]; val >>= 4; }
    out[i] = '\0'; reverse_str(out, i);
}
sigma_u64 binary_to_decimal(const char* s) {
    sigma_u64 r = 0; while (*s) r = (r << 1) | (*s++ - '0'); return r;
}
sigma_u64 octal_to_decimal(const char* s) {
    sigma_u64 r = 0; while (*s) r = r * 8 + (*s++ - '0'); return r;
}
sigma_u64 hex_to_decimal(const char* s) {
    if (s[0]=='0' && (s[1]=='x'||s[1]=='X')) s += 2;
    sigma_u64 r = 0;
    while (*s) {
        char c = *s++;
        sigma_u8 n = (c>='0'&&c<='9') ? (sigma_u8)(c-'0') :
                     (c>='A'&&c<='F') ? (sigma_u8)(c-'A'+10) :
                     (c>='a'&&c<='f') ? (sigma_u8)(c-'a'+10) : 0xFF;
        if (n == 0xFF) break;
        r = (r << 4) | n;
    }
    return r;
}

// --- Binary arithmetic ---
sigma_u64 ones_complement(sigma_u64 val, int bits) {
    sigma_u64 mask = (bits==64)?~0ULL:((1ULL<<bits)-1); return (~val)&mask;
}
sigma_u64 twos_complement(sigma_u64 val, int bits) {
    sigma_u64 mask = (bits==64)?~0ULL:((1ULL<<bits)-1);
    return (ones_complement(val,bits)+1)&mask;
}
sigma_u64 binary_add(sigma_u64 a, sigma_u64 b, bool* carry_out) {
    sigma_u64 r = a + b; if (carry_out) *carry_out = (r < a); return r;
}
sigma_u64 binary_subtract(sigma_u64 a, sigma_u64 b, bool* borrow_out) {
    if (borrow_out) *borrow_out = (a < b); return a - b;
}

// --- ASCII ---
bool is_valid_ascii(sigma_u8 code) { return code <= 127; }
sigma_u8 ascii_code(char c) { return (sigma_u8)c; }
const char* ascii_name(sigma_u8 code) {
    static const char* ctrl[] = {
        "NUL","SOH","STX","ETX","EOT","ENQ","ACK","BEL","BS","HT","LF",
        "VT","FF","CR","SO","SI","DLE","DC1","DC2","DC3","DC4","NAK",
        "SYN","ETB","CAN","EM","SUB","ESC","FS","GS","RS","US","SPC"
    };
    if (code <= 32) return ctrl[code];
    if (code == 127) return "DEL";
    return nullptr;
}

// --- BCD ---
sigma_u8  bcd_encode_digit(sigma_u8 d)  { return d & 0x0F; }
sigma_u8  bcd_decode_digit(sigma_u8 n)  { return n & 0x0F; }
sigma_u32 bcd_encode(sigma_u32 val) {
    sigma_u32 r = 0, sh = 0;
    while (val) { r |= (sigma_u32)(val % 10) << sh; val /= 10; sh += 4; }
    return r;
}
sigma_u32 bcd_decode(sigma_u32 bcd) {
    sigma_u32 r = 0, m = 1;
    while (bcd) { r += (bcd & 0xF) * m; bcd >>= 4; m *= 10; }
    return r;
}
sigma_u32 bcd_add(sigma_u32 a, sigma_u32 b) { return bcd_encode(bcd_decode(a)+bcd_decode(b)); }

// --- EBCDIC ---
const sigma_u8 EBCDIC_TO_ASCII_TABLE[256] = {
    0,1,2,3,156,9,134,127,151,141,142,11,12,13,14,15,
    16,17,18,19,157,133,8,135,24,25,146,143,28,29,30,31,
    128,129,130,131,132,10,23,27,136,137,138,139,140,5,6,7,
    144,145,22,147,148,149,150,4,152,153,154,155,20,21,158,26,
    32,160,161,162,163,164,165,166,167,168,91,46,60,40,43,33,
    38,169,170,171,172,173,174,175,176,177,93,36,42,41,59,94,
    45,47,178,179,180,181,182,183,184,185,124,44,37,95,62,63,
    186,187,188,189,190,191,192,193,194,96,58,35,64,39,61,34,
    195,97,98,99,100,101,102,103,104,105,196,197,198,199,200,201,
    202,106,107,108,109,110,111,112,113,114,203,204,205,206,207,208,
    209,126,115,116,117,118,119,120,121,122,210,211,212,213,214,215,
    216,217,218,219,220,221,222,223,224,225,226,227,228,229,230,231,
    123,65,66,67,68,69,70,71,72,73,232,233,234,235,236,237,
    125,74,75,76,77,78,79,80,81,82,238,239,240,241,242,243,
    92,159,83,84,85,86,87,88,89,90,244,245,246,247,248,249,
    48,49,50,51,52,53,54,55,56,57,250,251,252,253,254,255
};
sigma_u8 ebcdic_to_ascii(sigma_u8 e) { return EBCDIC_TO_ASCII_TABLE[e]; }
sigma_u8 ascii_to_ebcdic(sigma_u8 c) {
    for (int i = 0; i < 256; i++) if (EBCDIC_TO_ASCII_TABLE[i] == c) return (sigma_u8)i;
    return 0x3F;
}
void ebcdic_str_to_ascii(const sigma_u8* src, char* dst, sigma_usize len) {
    for (sigma_usize i = 0; i < len; i++) dst[i] = (char)ebcdic_to_ascii(src[i]);
}
void ascii_str_to_ebcdic(const char* src, sigma_u8* dst, sigma_usize len) {
    for (sigma_usize i = 0; i < len; i++) dst[i] = ascii_to_ebcdic((sigma_u8)src[i]);
}

// --- UTF-8 ---
bool is_valid_codepoint(sigma_u32 cp) {
    return cp <= 0x10FFFF && !(cp >= 0xD800 && cp <= 0xDFFF);
}
sigma_u32 utf8_decode_codepoint(const char** ptr, const char* end) {
    const sigma_u8* p = (const sigma_u8*)*ptr;
    sigma_u32 cp;
    if (*p < 0x80)           { cp = *p++; }
    else if ((*p&0xE0)==0xC0){ cp=((*p++&0x1F)<<6)|(*p++&0x3F); }
    else if ((*p&0xF0)==0xE0){ cp=((*p++&0x0F)<<12)|((*p++&0x3F)<<6)|(*p++&0x3F); }
    else                     { cp=((*p++&0x07)<<18)|((*p++&0x3F)<<12)|((*p++&0x3F)<<6)|(*p++&0x3F); }
    *ptr = (const char*)p;
    return cp;
}
int utf8_encode_codepoint(sigma_u32 cp, char* out, sigma_usize n) {
    if (cp<0x80&&n>=1)    { out[0]=(char)cp; return 1; }
    if (cp<0x800&&n>=2)   { out[0]=(char)(0xC0|(cp>>6)); out[1]=(char)(0x80|(cp&0x3F)); return 2; }
    if (cp<0x10000&&n>=3) { out[0]=(char)(0xE0|(cp>>12)); out[1]=(char)(0x80|((cp>>6)&0x3F)); out[2]=(char)(0x80|(cp&0x3F)); return 3; }
    if (n>=4)             { out[0]=(char)(0xF0|(cp>>18)); out[1]=(char)(0x80|((cp>>12)&0x3F)); out[2]=(char)(0x80|((cp>>6)&0x3F)); out[3]=(char)(0x80|(cp&0x3F)); return 4; }
    return -1;
}
sigma_usize utf8_strlen(const char* s) {
    sigma_usize c = 0; while (*s) { if ((*s&0xC0)!=0x80) c++; s++; } return c;
}

} // namespace Sigma::Codec
