/* SigmaOS ELF32 user program: units — unit conversion.
 * Usage: exec UNITS.ELF <number> <from> <to>
 * Families: length (m/ft/in/cm/mm), mass (kg/g/lb), temp (C/F/K, tenths),
 * time (s/min/h). Integer results except temps (1 decimal). */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static void outn_i(int v) { if (v < 0) { outs("-"); v = -v; } uputn((u32)v); }
static int eq(const char* a, const char* b) {
    while (*a && *a == *b) { a++; b++; }
    return *a == *b;
}
static int is_len(const char* u) {
    return eq(u,"m")||eq(u,"meter")||eq(u,"meters")||eq(u,"ft")||eq(u,"foot")||eq(u,"feet")
        || eq(u,"in")||eq(u,"inch")||eq(u,"inches")||eq(u,"cm")||eq(u,"mm");
}
static int conv_len(int v, const char* from, const char* to) {
    /* base: millimeters */
    int base;
    if (eq(from,"m")||eq(from,"meter")||eq(from,"meters")) base = v * 1000;
    else if (eq(from,"ft")||eq(from,"foot")||eq(from,"feet")) base = v * 304800 / 1000;
    else if (eq(from,"in")||eq(from,"inch")||eq(from,"inches")) base = v * 25400 / 1000;
    else if (eq(from,"cm")) base = v * 10;
    else if (eq(from,"mm")) base = v;
    else return -1;
    if (eq(to,"m")||eq(to,"meter")||eq(to,"meters")) return base / 1000;
    if (eq(to,"ft")||eq(to,"foot")||eq(to,"feet")) return base * 10 / 304800;
    if (eq(to,"in")||eq(to,"inch")||eq(to,"inches")) return base * 10 / 25400;
    if (eq(to,"cm")) return base / 10;
    if (eq(to,"mm")) return base;
    return -1;
}
static int is_mass(const char* u) { return eq(u,"kg")||eq(u,"g")||eq(u,"lb")||eq(u,"lbs"); }
static int conv_mass(int v, const char* from, const char* to) {
    int base;  /* grams */
    if (eq(from,"kg")) base = v * 1000;
    else if (eq(from,"g")) base = v;
    else if (eq(from,"lb")||eq(from,"lbs")) base = v * 453592 / 1000;
    else return -1;
    if (eq(to,"kg")) return base / 1000;
    if (eq(to,"g")) return base;
    if (eq(to,"lb")||eq(to,"lbs")) return base * 1000 / 453592;
    return -1;
}
static int is_temp(const char* u) { return eq(u,"C")||eq(u,"c")||eq(u,"F")||eq(u,"f")||eq(u,"K")||eq(u,"k"); }
static int conv_temp(int v, const char* from, const char* to) {
    /* returns tenths of degree C */
    int c10;
    if (eq(from,"C")||eq(from,"c")) c10 = v * 10;
    else if (eq(from,"F")||eq(from,"f")) c10 = (v - 32) * 50 / 9;
    else if (eq(from,"K")||eq(from,"k")) c10 = (v - 273) * 10;
    else return -1;
    if (eq(to,"C")||eq(to,"c")) return c10;
    if (eq(to,"F")||eq(to,"f")) return c10 * 9 / 5 + 320;
    if (eq(to,"K")||eq(to,"k")) return c10 + 2730;
    return -1;
}
static int is_time(const char* u) {
    return eq(u,"s")||eq(u,"sec")||eq(u,"secs")||eq(u,"m")||eq(u,"min")||eq(u,"mins")
        || eq(u,"h")||eq(u,"hr")||eq(u,"hrs");
}
static int conv_time(int v, const char* from, const char* to) {
    int base;  /* seconds */
    if (eq(from,"s")||eq(from,"sec")||eq(from,"secs")) base = v;
    else if (eq(from,"m")||eq(from,"min")||eq(from,"mins")) base = v * 60;
    else if (eq(from,"h")||eq(from,"hr")||eq(from,"hrs")) base = v * 3600;
    else return -1;
    if (eq(to,"s")||eq(to,"sec")||eq(to,"secs")) return base;
    if (eq(to,"m")||eq(to,"min")||eq(to,"mins")) return base / 60;
    if (eq(to,"h")||eq(to,"hr")||eq(to,"hrs")) return base / 3600;
    return -1;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 4) { uputs("usage: units <num> <from> <to>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int v = 0;
    for (const char* p = (const char*)sp[2]; *p >= '0' && *p <= '9'; p++) v = v * 10 + (*p - '0');
    const char* from = (const char*)sp[4];
    const char* to = (const char*)sp[6];
    int r = -1, temp = 0;
    if (is_len(from) && is_len(to)) r = conv_len(v, from, to);
    else if (is_mass(from) && is_mass(to)) r = conv_mass(v, from, to);
    else if (is_temp(from) && is_temp(to)) { r = conv_temp(v, from, to); temp = 1; }
    else if (is_time(from) && is_time(to)) r = conv_time(v, from, to);
    if (r < 0) { uputs("units: incompatible or unknown units\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    if (temp) {
        int neg = r < 0; if (neg) r = -r;
        if (neg) outs("-");
        outn_i(r / 10);
        int f = r % 10;
        if (f) { outs("."); uputn((u32)f); }
    } else {
        outn_i(r);
    }
    outs(" "); outs(to); outs("\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
