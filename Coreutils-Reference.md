# Coreutils Reference

SigmaOS bundles a sovereign, zero-overhead implementation of standard Unix/GNU utility commands implemented directly inside the kernel shell interface:

- **chmod**: Modify VFS resource node permissions (`coreutil_chmod`)
- **chown**: Change VFS resource owner/group (`coreutil_chown`)
- **cp**: Copy files (`coreutil_cp`)
- **mv**: Move or rename VFS entries (`coreutil_mv`)
- **touch**: Create files or update timestamps (`coreutil_touch`)
- **wc**: Word, line, and byte counter (`coreutil_wc`)
- **grep**: Search lines matching patterns (`coreutil_grep`)
- **head**: Output first N lines (`coreutil_head`)
- **tail**: Output last N lines (`coreutil_tail`)
- **df**: Display filesystem disk usage (`coreutil_df`)
- **du**: Estimate directory space usage (`coreutil_du`)

Refer to [sigma_coreutils.rs](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/kernel/core/sigma_coreutils.rs) for implementation details.
