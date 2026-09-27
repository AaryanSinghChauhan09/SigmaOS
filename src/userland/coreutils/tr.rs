//! tr - translate or delete characters
//! POSIX, GNU & BSD compatible tr implementation

use std::collections::HashSet;

/// Options for tr execution
#[derive(Debug, Clone, Default)]
pub struct TrOptions {
    pub delete: bool,
    pub squeeze: bool,
}

/// Translate, delete, or squeeze characters in input string
pub fn run(input: &str, set1: &str, set2: Option<&str>, opts: TrOptions) -> Result<String, String> {
    if opts.delete {
        let delete_set: HashSet<char> = set1.chars().collect();
        let filtered: String = input.chars().filter(|c| !delete_set.contains(c)).collect();

        if opts.squeeze {
            Ok(squeeze_string(&filtered, set1))
        } else {
            Ok(filtered)
        }
    } else if let Some(target_set) = set2 {
        let src_chars: Vec<char> = set1.chars().collect();
        let target_chars: Vec<char> = target_set.chars().collect();

        if src_chars.is_empty() {
            return Ok(input.to_string());
        }

        let last_target = *target_chars.last().unwrap_or(&' ');

        // Bolt optimization: Prebuild O(1) constant-time lookup maps instead of doing
        // an O(M) linear scan (.position(|&sc| sc == c)) on every character of input (O(N * M)).
        // Uses a 256-element direct array for ASCII characters and a HashMap for Unicode.
        let mut ascii_map: [Option<char>; 256] = [None; 256];
        let mut unicode_map = std::collections::HashMap::new();

        for (i, &sc) in src_chars.iter().enumerate() {
            let tc = *target_chars.get(i).unwrap_or(&last_target);
            let code = sc as usize;
            if code < 256 {
                if ascii_map[code].is_none() {
                    ascii_map[code] = Some(tc);
                }
            } else {
                unicode_map.entry(sc).or_insert(tc);
            }
        }

        let mut translated = String::with_capacity(input.len());
        for c in input.chars() {
            let code = c as usize;
            let mapped = if code < 256 {
                ascii_map[code].unwrap_or(c)
            } else {
                unicode_map.get(&c).copied().unwrap_or(c)
            };
            translated.push(mapped);
        }

        if opts.squeeze {
            Ok(squeeze_string(&translated, target_set))
        } else {
            Ok(translated)
        }
    } else if opts.squeeze {
        Ok(squeeze_string(input, set1))
    } else {
        Ok(input.to_string())
    }
}

/// Squeeze repeated consecutive characters in set
fn squeeze_string(input: &str, set: &str) -> String {
    let squeeze_set: HashSet<char> = set.chars().collect();
    // Bolt optimization: Pre-allocate String capacity to avoid reallocation overhead.
    let mut result = String::with_capacity(input.len());
    let mut last_char: Option<char> = None;

    for c in input.chars() {
        if squeeze_set.contains(&c) && last_char == Some(c) {
            continue;
        }
        result.push(c);
        last_char = Some(c);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tr_translate() {
        let res = run("hello world", "el", Some("ip"), TrOptions::default()).unwrap();
        assert_eq!(res, "hippo worpd");
    }

    #[test]
    fn test_tr_delete() {
        let opts = TrOptions {
            delete: true,
            squeeze: false,
        };
        let res = run("hello world", "lo", None, opts).unwrap();
        assert_eq!(res, "he wrd");
    }

    #[test]
    fn test_tr_translation_performance_and_unicode() {
        let large_input = "abcdefghijklmnopqrstuvwxyz".repeat(1000);
        let res = run(&large_input, "abcdefghijklmnopqrstuvwxyz", Some("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), TrOptions::default()).unwrap();
        assert_eq!(res.len(), large_input.len());
        assert!(res.starts_with("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));

        // Test unicode mapping
        let unicode_res = run("hello 🌍 world!", "🌍", Some("🌎"), TrOptions::default()).unwrap();
        assert_eq!(unicode_res, "hello 🌎 world!");
    }
}
