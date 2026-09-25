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

        let translated: String = input
            .chars()
            .map(|c| {
                if let Some(pos) = src_chars.iter().position(|&sc| sc == c) {
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
}
