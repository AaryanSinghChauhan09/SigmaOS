import re

file_path = 'kernel/core/memory_manager.cpp'
with open(file_path, 'r', encoding='utf-8') as f:
    content = f.read()

new_content = content.replace(
    'if (m_segments[i].start_addr == addr && m_segments[i].allocated) {',
    '''if (m_segments[i].start_addr == addr) {
            if (!m_segments[i].allocated) {
                sigma_log_info("[MEM FATAL]: Sovereign Double-Free Detected at %p!\\n", ptr);
                return;
            }'''
)

with open(file_path, 'w', encoding='utf-8') as f:
    f.write(new_content)

print('Patched memory_manager.cpp')
