/**
 * sigma_codec.h — SovereignCodec: Number Systems & Encoding Module
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-FCIT Unit I (Number Systems, Binary Arithmetic, Computer Codes)
 * Implements: Base conversions, binary arithmetic, ASCII/BCD/EBCDIC/Unicode support
 *
 * Zero-dependency, silicon-direct. No libc includes.
 */
#pragma once
#include "sigma_kernel_types.h"

namespace Sigma::Codec {

// ─── Number System Conversions ───────────────────────────────────────────────

    /** Convert decimal to binary string (result in caller-provided buffer) */
    void decimal_to_binary(sigma_u64 val, char* out, sigma_usize out_len);

    /** Convert decimal to octal string */
    void decimal_to_octal(sigma_u64 val, char* out, sigma_usize out_len);

    /** Convert decimal to hexadecimal string */
    void decimal_to_hex(sigma_u64 val, char* out, sigma_usize out_len, bool uppercase = true);

    /** Convert binary string (e.g., "1101") to decimal */
    sigma_u64 binary_to_decimal(const char* bin_str);

    /** Convert octal string to decimal */
    sigma_u64 octal_to_decimal(const char* oct_str);

    /** Convert hex string (e.g., "FF" or "0xFF") to decimal */
    sigma_u64 hex_to_decimal(const char* hex_str);

    /** Generic base conversion: any base 2–36 */
    sigma_u64 convert_base(sigma_u64 val, int from_base, int to_base,
                           char* out, sigma_usize out_len);

// ─── Binary Arithmetic ────────────────────────────────────────────────────────

    /** 1's complement: flip all bits in n-bit number */
    sigma_u64 ones_complement(sigma_u64 val, int bits);

    /** 2's complement: ones_complement + 1 */
    sigma_u64 twos_complement(sigma_u64 val, int bits);

    /** Binary addition with carry detection */
    sigma_u64 binary_add(sigma_u64 a, sigma_u64 b, bool* carry_out = nullptr);

    /** Binary subtraction using 2's complement method */
    sigma_u64 binary_subtract(sigma_u64 a, sigma_u64 b, bool* borrow_out = nullptr);

    /** Detect signed overflow in n-bit addition */
    bool detect_overflow(sigma_i64 a, sigma_i64 b, sigma_i64 result, int bits);

// ─── ASCII ────────────────────────────────────────────────────────────────────

    /** Look up ASCII character value (0–127) */
    bool is_valid_ascii(sigma_u8 code);
    const char* ascii_name(sigma_u8 code);   // e.g., 65 → "A", 10 → "LF"
    sigma_u8    ascii_code(char c);           // e.g., 'A' → 65

// ─── BCD (Binary Coded Decimal) ───────────────────────────────────────────────

    /** Encode single decimal digit (0–9) to 4-bit BCD */
    sigma_u8  bcd_encode_digit(sigma_u8 decimal_digit);

    /** Decode 4-bit BCD nibble to decimal digit */
    sigma_u8  bcd_decode_digit(sigma_u8 bcd_nibble);

    /** Encode full integer to packed BCD (2 digits per byte) */
    sigma_u32 bcd_encode(sigma_u32 decimal_val);

    /** Decode packed BCD to integer */
    sigma_u32 bcd_decode(sigma_u32 bcd_val);

    /** BCD addition (handles carry between nibbles) */
    sigma_u32 bcd_add(sigma_u32 a, sigma_u32 b);

// ─── EBCDIC ───────────────────────────────────────────────────────────────────

    /**
     * EBCDIC (Extended Binary Coded Decimal Interchange Code)
     * IBM legacy encoding — 256-character 8-bit code
     * Used in: IBM mainframe compatibility layer
     */
    extern const sigma_u8 EBCDIC_TO_ASCII_TABLE[256];
    extern const sigma_u8 ASCII_TO_EBCDIC_TABLE[256];

    sigma_u8  ascii_to_ebcdic(sigma_u8 ascii_char);
    sigma_u8  ebcdic_to_ascii(sigma_u8 ebcdic_char);
    void      ascii_str_to_ebcdic(const char* src, sigma_u8* dst, sigma_usize len);
    void      ebcdic_str_to_ascii(const sigma_u8* src, char* dst, sigma_usize len);

// ─── Unicode / UTF-8 / UTF-16 / UTF-32 ───────────────────────────────────────

    /**
     * Unicode code point range: U+0000 to U+10FFFF
     * UTF-8: variable width 1-4 bytes (ASCII-compatible)
     * UTF-16: 2 or 4 bytes (surrogate pairs for > U+FFFF)
     * UTF-32: fixed 4 bytes per code point
     */

    /** Decode one UTF-8 sequence → code point; advances ptr */
    sigma_u32 utf8_decode_codepoint(const char** ptr, const char* end);

    /** Encode code point → UTF-8 bytes; returns bytes written */
    int       utf8_encode_codepoint(sigma_u32 codepoint, char* out, sigma_usize out_len);

    /** UTF-8 string length in code points (not bytes) */
    sigma_usize utf8_strlen(const char* utf8_str);

    /** Convert UTF-8 string to UTF-16 LE */
    int utf8_to_utf16(const char* src, sigma_u16* dst, sigma_usize dst_len);

    /** Convert UTF-16 LE to UTF-8 */
    int utf16_to_utf8(const sigma_u16* src, char* dst, sigma_usize dst_len);

    /** Code point → UTF-32 (trivial: same value) */
    inline sigma_u32 codepoint_to_utf32(sigma_u32 cp) { return cp; }

    /** Check if code point is valid Unicode */
    bool is_valid_codepoint(sigma_u32 cp);

    /** HTML entity encoding: codepoint → &#xHH; */
    int codepoint_to_html_entity(sigma_u32 cp, char* out, sigma_usize out_len);

// ─── Multi-language Formatter ─────────────────────────────────────────────────

    /**
     * Format integer in the style expected by each language.
     * Used by sigma-cli: `sigma convert 255 --lang py`
     */
    enum class LangFormat { C_CPP, PYTHON, JAVASCRIPT, PHP, HTML, JAVA };

    void format_for_lang(sigma_u64 val, int base, LangFormat lang,
                         char* out, sigma_usize out_len);
    // C/C++:      0xFF       0b11111111  0377
    // Python:     0xFF  hex(255)  bin(255)  oct(255)
    // JavaScript: 0xFF       0b11111111  0o377
    // PHP:        0xFF       decbin(255)  decoct(255)
    // HTML:       &#xFF;     &#255;
    // Java:       0xFF       Integer.toBinaryString(255)

} // namespace Sigma::Codec
