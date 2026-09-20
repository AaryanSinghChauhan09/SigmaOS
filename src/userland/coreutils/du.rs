//! du - estimate file space usage
//! POSIX-compatible du implementation

use std::fs;
use std::path::Path;

/// Estimate disk usage
pub fn run(path: &str, _human_readable: bool, all: bool, summarize: bool) -> Result<Vec<(String, u64)>, String> {
    let path_obj = Path::new(path);
    
    if !path_obj.exists() {
        return Err(format!("du: cannot access '{}': No such file or directory", path));
    }
    
    let mut results = Vec::new();
    
    if path_obj.is_file() {
        let size = get_file_size(path_obj)?;
        results.push((path.to_string(), size));
    } else {
        calculate_directory_size(path_obj, &mut results, all, summarize)?;
    }
    
    Ok(results)
}

/// Calculate directory size recursively
fn calculate_directory_size(
    path: &Path,
    results: &mut Vec<(String, u64)>,
    all: bool,
    summarize: bool,
) -> Result<u64, String> {
    let mut total_size = 0u64;
    
    for entry in fs::read_dir(path).map_err(|e| format!("du: {}", e))? {
        let entry = entry.map_err(|e| format!("du: {}", e))?;
        let entry_path = entry.path();
        let entry_name = entry_path.to_string_lossy().to_string();
        
        if entry_path.is_dir() {
            let dir_size = calculate_directory_size(&entry_path, results, all, summarize)?;
            total_size += dir_size;
            
            if !summarize {
                results.push((entry_name, dir_size));
            }
        } else {
            let file_size = get_file_size(&entry_path)?;
            total_size += file_size;
            
            if all {
                results.push((entry_name, file_size));
            }
        }
    }
    
    if !summarize {
        results.push((path.to_string_lossy().to_string(), total_size));
    }
    
    Ok(total_size)
}

/// Get file size
fn get_file_size(path: &Path) -> Result<u64, String> {
    let metadata = fs::metadata(path).map_err(|e| format!("du: {}", e))?;
    Ok(metadata.len())
}

/// Format size for human-readable output
pub fn format_size(size: u64, human_readable: bool) -> String {
    if human_readable {
        const UNITS: &[&str] = &["B", "K", "M", "G", "T"];
        let mut size_f = size as f64;
        let mut unit_idx = 0;
        
        while size_f >= 1024.0 && unit_idx < UNITS.len() - 1 {
            size_f /= 1024.0;
            unit_idx += 1;
        }
        
        format!("{:.1}{}", size_f, UNITS[unit_idx])
    } else {
        format!("{}", size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_get_file_size() {
        let test_dir = format!("/tmp/test_du_{}", std::process::id());
        fs::create_dir_all(&test_dir).unwrap();

        let test_file = format!("{}/test.txt", test_dir);
        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"test content").unwrap();

        let size = get_file_size(Path::new(&test_file)).unwrap();
        assert_eq!(size, 12);

        fs::remove_dir_all(&test_dir).unwrap();
    }
}
