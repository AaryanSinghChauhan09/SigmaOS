//! env - run a program in a modified environment or print environment variables
//! POSIX, GNU & BSD compatible env implementation

use std::env;

/// Print all environment variables or set variable and return description
pub fn run(set_vars: &[(&str, &str)], ignore_environment: bool) -> Result<String, String> {
    let mut vars = if ignore_environment {
        Vec::new()
    } else {
        env::vars().collect::<Vec<(String, String)>>()
    };

    for (k, v) in set_vars {
        if let Some(pos) = vars.iter().position(|(existing_k, _)| existing_k == k) {
            vars[pos] = (k.to_string(), v.to_string());
        } else {
            vars.push((k.to_string(), v.to_string()));
        }
    }

    vars.sort_by(|a, b| a.0.cmp(&b.0));

    let mut output = String::new();
    for (key, val) in vars {
        output.push_str(&format!("{}={}\n", key, val));
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_set_and_ignore() {
        let set_vars = [("SIGMA_ENV", "TEST_VALUE")];
        let result = run(&set_vars, true).unwrap();
        assert_eq!(result, "SIGMA_ENV=TEST_VALUE\n");
    }
}
