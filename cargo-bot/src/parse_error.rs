use regex::Regex;

pub fn parse_errors(output: &str) -> Vec<String> {
    let error_start_re = Regex::new(r"^\s*error").unwrap();

    let mut errors = output.lines().fold(Vec::new(), |mut acc, line| {
        if error_start_re.is_match(line) {
            acc.push(String::new());
        }

        if !acc.is_empty() {
            acc.last_mut().unwrap().push_str(line);
        }

        acc
    });

    // The last error is always empty just a descirption of the failure so we want to ignore it
    errors.pop();

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_single_error() {
        let output = "
        Checking crate v1.0.0 (/path/to/crate)
        error[E0412]: cannot find type `u16s` in this scope
        --> src/config.rs:14:15
        |
        14 |     pub port: u16s,
        |               ^^^^ help: a builtin type with a similar name exists: `u16`";

        let errors = parse_errors(output);
        assert_eq!(errors.len(), 1, "expected 2 errors, got {:?}", errors);
        assert!(errors[0].contains("error[E0412]: cannot find type `u16s` in this scope"));
    }

    #[test]
    fn it_parses_multiple_errors() {
        let output = "
        Checking crate v1.0.0 (/path/to/crate)
        error[E0412]: cannot find type `u16s` in this scope
        --> src/config.rs:14:15
        |
        14 |     pub port: u16s,
        |               ^^^^ help: a builtin type with a similar name exists: `u16`
        
        error[E0412]: cannot find type `u16s` in this scope
        --> src/config.rs:14:15
        |
        14 |     pub port: u16s,
        |               ^^^^
        |
        help: a builtin type with a similar name exists
        |
        14 |     pub port: u16,
        |               ~~~
        help: you might be missing a type parameter
        |
        12 | pub struct WebhookConfig<u16s> {";

        let errors = parse_errors(output);
        assert_eq!(errors.len(), 2, "expected 2 errors, got {:?}", errors);
        assert!(
            errors[0].contains("error[E0412]: cannot find type `u16s` in this scope"),
            "{}",
            errors[0]
        );
        assert!(
            errors[1].contains("error[E0412]: cannot find type `u16s` in this scope"),
            "{}",
            errors[1]
        );
    }

    #[test]
    fn it_ignores_non_error_output() {
        let output = "
        Compiling crate v1.0.0 (/path/to/crate)
        Finished dev [unoptimized + debuginfo] target(s) in 0.63s";

        let errors = parse_errors(output);
        assert_eq!(errors.len(), 0);
    }
}
