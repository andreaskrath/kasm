use crate::{constant::COMMENT, error::DataError};

const DATA_SECTION: &str = "data:";

pub fn expand_data_section(s: &str) -> Result<String, DataError> {
    if let Some(data_section_start) = s.rfind(DATA_SECTION) {
        let (program, data) = s
            .split_at_checked(data_section_start)
            .ok_or(DataError::Encoding)?;

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
                return Err(DataError::InvalidKeyFormat(key.to_string()));
            }

            let value = split
                .next()
                .ok_or(DataError::MissingValue(key.to_string()))?;

            program = program.replace(key, value);
        }

        program.shrink_to_fit();

        Ok(program)
    } else {
        Ok(s.to_string())
    }
}

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
    use crate::{error::DataError, preprocess::expand_data_section};

    #[test]
    fn invalid_key_format() {
        let input = ["pshb one", "stop", "", "data:", "    one 1"].join("\n");
        let expected = Err(DataError::InvalidKeyFormat("one".to_string()));

        let actual = expand_data_section(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn no_data_section() -> Result<(), DataError> {
        let input = ["setb ra 10", "pshw 200", "stop"].join("\n");

        let actual = expand_data_section(&input)?;

        assert_eq!(actual, input);

        Ok(())
    }

    #[test]
    fn data_section() -> Result<(), DataError> {
        let input = ["pshb NUMBER_1", "stop", "", "data:", "  NUMBER_1 1"].join("\n");
        let expected = ["pshb 1", "stop"].join("\n");

        let actual = expand_data_section(&input)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn data_section_with_comment() -> Result<(), DataError> {
        let input = [
            "pshb NUMBER_1",
            "stop",
            "",
            "data:",
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
    fn data_section_with_empty_line() -> Result<(), DataError> {
        let input = [
            "pshb NUMBER_1",
            "pshb NUMBER_2",
            "stop",
            "",
            "data:",
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
