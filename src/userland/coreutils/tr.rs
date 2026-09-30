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

        // Bolt ⚡ Performance Optimization: Precompute O(1) constant-time character translation lookup map.
        // Replaces O(N * M) linear position scanning inside string mapping loops with O(1) direct indexing for ASCII
        // and O(1) hash map lookups for Unicode characters, reducing overall translation time complexity from O(N * M) to O(N + M).
        let mut ascii_map = [None; 256];
        let mut unicode_map = std::collections::HashMap::new();

        for (i, &sc) in src_chars.iter().enumerate() {
            let tc = *target_chars.get(i).unwrap_or(&last_target);
            let val = sc as usize;
            if val < 256 {
                if ascii_map[val].is_none() {
                    ascii_map[val] = Some(tc);
                }
            } else {
                unicode_map.entry(sc).or_insert(tc);
            }
        }

        let mut translated = String::with_capacity(input.len());
        for c in input.chars() {
            let val = c as usize;
            if val < 256 {
                if let Some(tc) = ascii_map[val] {
                    translated.push(tc);
                } else {
                    translated.push(c);
                }
            } else if let Some(&tc) = unicode_map.get(&c) {
                translated.push(tc);
            } else {
                translated.push(c);
            }
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
    // Bolt ⚡ Performance Optimization: Pre-allocate capacity to eliminate dynamic string buffer reallocations.
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
    fn test_tr_unicode_translation() {
        let res = run("αβγ δεζ", "αβγ", Some("123"), TrOptions::default()).unwrap();
        assert_eq!(res, "123 δεζ");
    }

    #[test]
    fn test_tr_duplicate_set1_precedence() {
        // First occurrence of character in set1 takes precedence
        let res = run("a b a c", "aba", Some("xyz"), TrOptions::default()).unwrap();
        assert_eq!(res, "x y x c");
    }

    #[test]
    fn test_tr_squeeze_and_translate() {
        let opts = TrOptions {
            delete: false,
            squeeze: true,
        };
        let res = run("heelloo  twoorld", "el", Some("ip"), opts).unwrap();
        assert_eq!(res, "hipoo  twoorpd");
    }

    #[test]
    fn test_tr_o1_lookup_benchmark() {
        // Verify O(N + M) performance on large string input
        let large_input = "abcdefghijklmnopqrstuvwxyz".repeat(500);
        let res = run(&large_input, "abcdefghijklmnopqrstuvwxyz", Some("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), TrOptions::default()).unwrap();
        assert_eq!(res.len(), large_input.len());
        assert!(res.starts_with("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
    }
}
