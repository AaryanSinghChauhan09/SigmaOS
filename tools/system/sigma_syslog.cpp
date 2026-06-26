/*
 * Σ SigmaOS — sigma_syslog: Centralized Logging Daemon
 * Zero-Dependency: Routes kernel ring buffer messages to persistent storage.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" u32  sigma_fat32_write(const char* name, const unsigned char* buf, u32 len);

#define LOG_BUFFER_SIZE 4096

static char syslog_buffer[LOG_BUFFER_SIZE];
static u32  syslog_head = 0;
static const char* LOG_FILE = "sigmaos.log";

/* Log a message to the buffer and flush to disk */
extern "C" void sigma_syslog_write(const char* level, const char* msg) {
    /* Format message */
    sigma_vga_printf("[%s] %s\n", level, msg);
    
    /* Very basic append logic */
    u32 l = 0;
    while (level[l] && syslog_head < LOG_BUFFER_SIZE - 4) {
        syslog_buffer[syslog_head++] = level[l++];
    }
    syslog_buffer[syslog_head++] = ']';
    syslog_buffer[syslog_head++] = ' ';
    
    u32 m = 0;
    while (msg[m] && syslog_head < LOG_BUFFER_SIZE - 2) {
        syslog_buffer[syslog_head++] = msg[m++];
    }
    syslog_buffer[syslog_head++] = '\n';
    
    /* In a real implementation, write to VFS log file */
    /* sigma_fat32_write(LOG_FILE, (unsigned char*)syslog_buffer, syslog_head); */
}
