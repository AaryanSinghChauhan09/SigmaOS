#!/usr/bin/env python3
import sys
import os
import stat

def create_cpio_header(filename, st, content_len):
    # CPIO new ASCII format (070701)
    magic = b"070701"
    ino = f"{st.st_ino:08X}".encode('ascii')
    mode = f"{st.st_mode:08X}".encode('ascii')
    uid = f"{st.st_uid:08X}".encode('ascii')
    gid = f"{st.st_gid:08X}".encode('ascii')
    nlink = f"{st.st_nlink:08X}".encode('ascii')
    mtime = f"{int(st.st_mtime):08X}".encode('ascii')
    filesize = f"{content_len:08X}".encode('ascii')
    devmajor = f"{os.major(st.st_dev):08X}".encode('ascii')
    devminor = f"{os.minor(st.st_dev):08X}".encode('ascii')
    rdevmajor = f"{os.major(st.st_rdev):08X}".encode('ascii')
    rdevminor = f"{os.minor(st.st_rdev):08X}".encode('ascii')
    
    name_bytes = filename.encode('utf-8') + b'\x00'
    namesize = f"{len(name_bytes):08X}".encode('ascii')
    check = b"00000000"
    
    header = (magic + ino + mode + uid + gid + nlink + mtime + filesize + 
              devmajor + devminor + rdevmajor + rdevminor + namesize + check)
    
    # Pad name to 4 bytes boundary including the header size (110 bytes)
    pad_len = (4 - ((110 + len(name_bytes)) % 4)) % 4
    return header + name_bytes + (b'\x00' * pad_len)

def pack_initramfs(source_dir, out_file):
    with open(out_file, 'wb') as f:
        for root, dirs, files in os.walk(source_dir):
            for name in files:
                filepath = os.path.join(root, name)
                arcname = os.path.relpath(filepath, source_dir)
                
                st = os.stat(filepath)
                with open(filepath, 'rb') as inf:
                    content = inf.read()
                
                header = create_cpio_header(arcname, st, len(content))
                f.write(header)
                f.write(content)
                
                # Pad data
                data_pad = (4 - (len(content) % 4)) % 4
                f.write(b'\x00' * data_pad)
                
        # Write TRAILER!!!
        class DummyStat:
            st_ino = 0; st_mode = 0; st_uid = 0; st_gid = 0; st_nlink = 1; st_mtime = 0
            st_dev = 0; st_rdev = 0
        trailer_header = create_cpio_header("TRAILER!!!", DummyStat(), 0)
        f.write(trailer_header)

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: sigma_mkinitfs.py <source_dir> <output.cpio>")
        sys.exit(1)
    pack_initramfs(sys.argv[1], sys.argv[2])
