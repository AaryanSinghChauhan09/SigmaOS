/* POSIX Shim Stub */
int posix_open(const char* path, int flags) {
    return -1; // Fallback to sovereign open
}
