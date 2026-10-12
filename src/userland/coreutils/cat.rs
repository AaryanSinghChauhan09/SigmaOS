//! cat - concatenate files and print on standard output
//! POSIX-compatible cat implementation

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Concatenate and print files
pub fn run(paths: &[String], show_number: bool, show_ends: bool) -> Result<(), String> {
    for path in paths {
        let path_obj = Path::new(path);

        if path == "-" {
            // Read from stdin
            let stdin = io::stdin();
            let stdout = io::stdout();
            copy_with_options(
                &mut stdin.lock(),
                &mut stdout.lock(),
                show_number,
                show_ends,
            )?;
        } else {
            let mut file = File::open(path_obj).map_err(|e| format!("cat: {}: {}", path, e))?;
            let stdout = io::stdout();
            let mut stdout_lock = stdout.lock();
            copy_with_options(&mut file, &mut stdout_lock, show_number, show_ends)?;
        }
    }

    Ok(())
}

/// Copy with line numbering and end-of-line markers
fn copy_with_options<R: Read, W>(
    reader: &mut R,
    writer: &mut W,
    show_number: bool,
    show_ends: bool,
) -> Result<(), String>
where
    W: std::io::Write,
{
    let mut buffer = [0u8; 8192];
    let mut line_number = 1;

    loop {
        let n = reader
            .read(&mut buffer)
            .map_err(|e| format!("cat: {}", e))?;
        if n == 0 {
            break;
        }

        if show_number {
            let line_str = format!("{:6} ", line_number);
            writer
                .write_all(line_str.as_bytes())
                .map_err(|e| format!("cat: {}", e))?;
            line_number += 1;
        }

        if show_ends {
            // Performance optimization: Pre-allocate buffer capacity to avoid per-newline
            // dynamic vector allocations (vec![b'$', b'\n']) and repeated heap allocations in flat_map.
            let mut modified = Vec::with_capacity(n + 128);
            for &b in &buffer[..n] {
                if b == b'\n' {
                    modified.push(b'$');
                }
                modified.push(b);
            }
            writer
                .write_all(&modified)
                .map_err(|e| format!("cat: {}", e))?;
        } else {
            writer
                .write_all(&buffer[..n])
                .map_err(|e| format!("cat: {}", e))?;
        }
    }

    Ok(())
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_cat_basic() {
        let input = "test content\nline 2\n";
        let mut reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();

        copy_with_options(&mut reader, &mut output, false, false).unwrap();
        assert_eq!(String::from_utf8_lossy(&output), input);
    }

    #[test]
    fn test_cat_show_ends() {
        let input = "hello\nworld\n";
        let mut reader = Cursor::new(input.as_bytes());
        let mut output = Vec::new();

        copy_with_options(&mut reader, &mut output, false, true).unwrap();
        assert_eq!(String::from_utf8_lossy(&output), "hello$\nworld$\n");
    }
}
