use std::collections::HashMap;

use crate::{
    constant::{Word, COMMENT},
    decode::DECODE_TABLE,
    error::PreProcessError,
};

const DATA_SECTION: &str = "DATA:";

pub fn expand_data_section(s: &str) -> Result<String, PreProcessError> {
    if let Some(data_section_start) = s.rfind(DATA_SECTION) {
        let (program, data) = s
            .split_at_checked(data_section_start)
            .expect("this should always be a valid split because its ascii");

        let mut program = program.trim_end().to_string();
        for line in data.lines().skip(1) {
            let line = line.trim_start();
            if line.is_empty() || line.starts_with(COMMENT) {
                continue;
            }

            let mut split = line.split_ascii_whitespace();
            let key = split
                .next()
                .expect("due to the line not being empty, there should always be at least a key");

            if !is_screaming_snake_case(key) {
                return Err(PreProcessError::InvalidKeyFormat(key.to_string()));
            }

            let value = split
                .next()
                .ok_or(PreProcessError::MissingValue(key.to_string()))?;

            program = program.replace(key, value);
        }

        program.shrink_to_fit();

        Ok(program)
    } else {
        Ok(s.to_string())
    }
}

// This implementation is quite poor IMO - ideally this is changed to something clearer
// and more performant in the future. Test cases are written so similar behaviour can be
// asserted with a new implementation.
pub fn expand_function_calls(s_program: String) -> Result<Box<[String]>, PreProcessError> {
    let mut program: Vec<String> = s_program.lines().map(|l| l.to_string()).collect();
    let mut f_index = HashMap::new();

    // index function named and where they should map to
    let p_clone = program.clone();
    for (line_number, line) in p_clone.iter().enumerate() {
        let line = line.trim_start();
        if line.is_empty() || line.starts_with(COMMENT) {
            continue;
        }

        let mut line_iter = line.split_ascii_whitespace();
        if line_iter.next().is_some_and(|s| s == "fn") {
            let f_name = line_iter
                .next()
                .ok_or(PreProcessError::MissingFunctionName)?;
            let f_name = f_name
                .strip_suffix(':')
                .ok_or(PreProcessError::MissingColonSuffix)?;

            if !is_snake_case(f_name) {
                return Err(PreProcessError::InvalidFunctionNameFormat(
                    f_name.to_string(),
                ));
            }

            if DECODE_TABLE.get(f_name).is_some() {
                return Err(PreProcessError::FunctionNamedAfterInstruction);
            }

            // adding 2 because we are one-indexing the source code
            if f_index.insert(f_name, line_number + 2).is_some() {
                return Err(PreProcessError::DuplicateFunctionName(f_name.to_string()));
            }
        }
    }

    // substitute function named for line number at call sites
    for line in program.iter_mut() {
        let trim = line.trim_start();
        if trim.is_empty() || trim.starts_with(COMMENT) {
            continue;
        }

        let mut trim_iter = trim.split_ascii_whitespace();
        if trim_iter.next().is_some_and(|s| s == "call") {
            if let Some(possible_function) = trim_iter.next() {
                let Some(destination) = f_index.get(possible_function) else {
                    if possible_function.parse::<Word>().is_ok() {
                        continue;
                    }

                    return Err(PreProcessError::UndefinedFunctionCalled(
                        possible_function.to_string(),
                    ));
                };

                *line = line.replacen(possible_function, &destination.to_string(), 1);
            }
        }
    }

    Ok(program.into_boxed_slice())
}

#[inline]
fn is_snake_case(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

#[inline]
fn is_screaming_snake_case(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

#[cfg(test)]
mod is_screaming_snake_case {
    use crate::preprocess::is_screaming_snake_case;

    #[test]
    fn lowercase() {
        let input = "one";
        assert!(!is_screaming_snake_case(input));
    }

    #[test]
    fn uppercase() {
        let input = "ONE";
        assert!(is_screaming_snake_case(input));
    }

    #[test]
    fn lowercase_with_underscores() {
        let input = "number_one";
        assert!(!is_screaming_snake_case(input));
    }

    #[test]
    fn uppercase_with_underscores() {
        let input = "NUMBER_ONE";
        assert!(is_screaming_snake_case(input));
    }

    #[test]
    fn lowercase_with_digits() {
        let input = "number1";
        assert!(!is_screaming_snake_case(input));
    }

    #[test]
    fn uppercase_with_digits() {
        let input = "NUMBER1";
        assert!(is_screaming_snake_case(input));
    }

    #[test]
    fn lowercase_with_underscores_and_digits() {
        let input = "number_1";
        assert!(!is_screaming_snake_case(input));
    }

    #[test]
    fn uppercase_with_underscores_and_digits() {
        let input = "NUMBER_1";
        assert!(is_screaming_snake_case(input));
    }
}

#[cfg(test)]
mod expand_data_section {
    use crate::{error::PreProcessError, preprocess::expand_data_section};

    #[test]
    fn invalid_key_format() {
        let input = ["pshb one", "stop", "", "DATA:", "    one 1"].join("\n");
        let expected = Err(PreProcessError::InvalidKeyFormat("one".to_string()));

        let actual = expand_data_section(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_data_section() -> Result<(), PreProcessError> {
        let input = ["setb ra 10", "pshw 200", "stop"].join("\n");

        let actual = expand_data_section(&input)?;

        assert_eq!(actual, input);

        Ok(())
    }

    #[test]
    fn data_section() -> Result<(), PreProcessError> {
        let input = ["pshb NUMBER_1", "stop", "", "DATA:", "  NUMBER_1 1"].join("\n");
        let expected = ["pshb 1", "stop"].join("\n");

        let actual = expand_data_section(&input)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn data_section_with_comment() -> Result<(), PreProcessError> {
        let input = [
            "pshb NUMBER_1",
            "stop",
            "",
            "DATA:",
            "  NUMBER_1 1",
            "  // this is a comment",
        ]
        .join("\n");
        let expected = ["pshb 1", "stop"].join("\n");

        let actual = expand_data_section(&input)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn data_section_with_empty_line() -> Result<(), PreProcessError> {
        let input = [
            "pshb NUMBER_1",
            "pshb NUMBER_2",
            "stop",
            "",
            "DATA:",
            "  NUMBER_1 1",
            "  ",
            "  NUMBER_2 2",
        ]
        .join("\n");
        let expected = ["pshb 1", "pshb 2", "stop"].join("\n");

        let actual = expand_data_section(&input)?;

        assert_eq!(actual, expected);

        Ok(())
    }
}

#[cfg(test)]
mod is_snake_case {
    use crate::preprocess::is_snake_case;

    #[test]
    fn lowercase() {
        let input = "one";
        assert!(is_snake_case(input));
    }

    #[test]
    fn uppercase() {
        let input = "ONE";
        assert!(!is_snake_case(input));
    }

    #[test]
    fn lowercase_with_underscores() {
        let input = "number_one";
        assert!(is_snake_case(input));
    }

    #[test]
    fn uppercase_with_underscores() {
        let input = "NUMBER_ONE";
        assert!(!is_snake_case(input));
    }

    #[test]
    fn lowercase_with_digits() {
        let input = "number1";
        assert!(is_snake_case(input));
    }

    #[test]
    fn uppercase_with_digits() {
        let input = "NUMBER1";
        assert!(!is_snake_case(input));
    }

    #[test]
    fn lowercase_with_underscores_and_digits() {
        let input = "number_1";
        assert!(is_snake_case(input));
    }

    #[test]
    fn uppercase_with_underscores_and_digits() {
        let input = "NUMBER_1";
        assert!(!is_snake_case(input));
    }
}

#[cfg(test)]
mod expand_function_calls {
    use crate::{error::PreProcessError, preprocess::expand_function_calls};

    #[test]
    fn missing_function_name_error() {
        let input = ["fn", "addq ra 1"].join("\n");
        let expected = Err(PreProcessError::MissingFunctionName);

        let actual = expand_function_calls(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn missing_colon_suffix_error() {
        let input = ["fn add_number", "addq ra 1"].join("\n");
        let expected = Err(PreProcessError::MissingColonSuffix);

        let actual = expand_function_calls(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn duplicate_function_name_error() {
        let input = [
            "fn add_number:",
            "addq ra 1",
            "ret",
            "",
            "fn add_number:",
            "addq ra 1",
            "ret",
        ]
        .join("\n");
        let expected = Err(PreProcessError::DuplicateFunctionName(
            "add_number".to_string(),
        ));

        let actual = expand_function_calls(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn undefined_function_called_error() {
        let input = ["fn add_number:", "addq ra 1", "ret", "", "call sub_number"].join("\n");
        let expected = Err(PreProcessError::UndefinedFunctionCalled(
            "sub_number".to_string(),
        ));

        let actual = expand_function_calls(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn invalid_function_name_format_error() {
        let input = ["fn ADD_NUMBER:", "addq ra 1", "ret"].join("\n");
        let expected = Err(PreProcessError::InvalidFunctionNameFormat(
            "ADD_NUMBER".to_string(),
        ));

        let actual = expand_function_calls(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn function_named_after_instruction_error() {
        let input = ["fn addq:", "addq ra 1", "ret"].join("\n");
        let expected = Err(PreProcessError::FunctionNamedAfterInstruction);

        let actual = expand_function_calls(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_function_definition() -> Result<(), PreProcessError> {
        let input = ["fn inc_ra:", "  addb ra 1", "  ret"].join("\n");
        let expected: Box<[String]> = Box::new([
            String::from("fn inc_ra:"),
            String::from("  addb ra 1"),
            String::from("  ret"),
        ]);

        let actual = expand_function_calls(input)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn valid_substitution_and_call() -> Result<(), PreProcessError> {
        let input = ["fn inc_ra:", "  addb ra 1", "  ret", "", "call inc_ra"].join("\n");
        let expected: Box<[String]> = Box::new([
            String::from("fn inc_ra:"),
            String::from("  addb ra 1"),
            String::from("  ret"),
            String::new(),
            String::from("call 2"),
        ]);

        let actual = expand_function_calls(input)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn call_before_function_definition() -> Result<(), PreProcessError> {
        let input = ["call inc_ra", "", "fn inc_ra:", "  addb ra 1", "  ret"].join("\n");
        let expected: Box<[String]> = Box::new([
            String::from("call 4"),
            String::new(),
            String::from("fn inc_ra:"),
            String::from("  addb ra 1"),
            String::from("  ret"),
        ]);

        let actual = expand_function_calls(input)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn multiple_substitutions_and_calls() -> Result<(), PreProcessError> {
        let input = [
            "fn inc_ra:",
            "  addb ra 1",
            "  ret",
            "",
            "fn sub_ra:",
            "  subb ra 1",
            "  ret",
            "",
            "call inc_ra",
            "call sub_ra",
        ]
        .join("\n");
        let expected: Box<[String]> = Box::new([
            String::from("fn inc_ra:"),
            String::from("  addb ra 1"),
            String::from("  ret"),
            String::new(),
            String::from("fn sub_ra:"),
            String::from("  subb ra 1"),
            String::from("  ret"),
            String::new(),
            String::from("call 2"),
            String::from("call 6"),
        ]);

        let actual = expand_function_calls(input)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn substitution_and_call_with_comments() -> Result<(), PreProcessError> {
        let input = [
            "// increments register a",
            "fn inc_ra:",
            "  // adds one to register a",
            "  addb ra 1",
            "  ret",
            "",
            "call inc_ra",
        ]
        .join("\n");
        let expected: Box<[String]> = Box::new([
            String::from("// increments register a"),
            String::from("fn inc_ra:"),
            String::from("  // adds one to register a"),
            String::from("  addb ra 1"),
            String::from("  ret"),
            String::new(),
            String::from("call 3"),
        ]);

        let actual = expand_function_calls(input)?;

        assert_eq!(actual, expected);

        Ok(())
    }
}
