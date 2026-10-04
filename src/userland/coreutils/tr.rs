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
        // Keep ASCII lookups allocation-free, and use a map for the remaining
        // Unicode characters. `or_insert` preserves tr's first-match behavior
        // when set1 contains duplicates.
        let mut ascii_map = [None; 128];
        let mut unicode_map = std::collections::HashMap::new();
        for (index, source) in src_chars.iter().copied().enumerate() {
            if source.is_ascii() {
                let slot = &mut ascii_map[source as usize];
                if slot.is_none() {
                    *slot = Some(index);
                }
            } else {
                unicode_map.entry(source).or_insert(index);
            }
        }

        let translated: String = input
            .chars()
            .map(|c| {
                let pos = if c.is_ascii() {
                    ascii_map[c as usize]
                } else {
                    unicode_map.get(&c).copied()
                };
                if let Some(pos) = pos {
                    *target_chars.get(pos).unwrap_or(&last_target)
                } else {
                    c
                }
            })
            .collect();

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
    let mut result = String::new();
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
    fn translation_preserves_first_duplicate_source_mapping() {
        let res = run("a", "aba", Some("xyz"), TrOptions::default()).unwrap();
        assert_eq!(res, "x");
    }

    #[test]
    fn translation_handles_unicode_sets() {
        let res = run("λ🙂x", "λ🙂", Some("αβ"), TrOptions::default()).unwrap();
        assert_eq!(res, "αβx");
    }

    #[test]
    fn translation_uses_last_target_for_short_target_set() {
        let res = run("abc", "abc", Some("x"), TrOptions::default()).unwrap();
        assert_eq!(res, "xxx");
    }
}
