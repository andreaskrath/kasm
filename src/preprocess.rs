use crate::error::DataError;

const DATA_SECTION: &str = "data:";

pub fn expand_data_section(s: &str) -> Result<String, DataError> {
    if let Some(data_section_start) = s.rfind(DATA_SECTION) {
        let (program, data) = s
            .split_at_checked(data_section_start)
            .ok_or(DataError::Encoding)?;

        let mut program = program.trim_end().to_string();
        for line in data.lines().skip(1) {
            if !line.is_empty() {
                let mut split = line.split_ascii_whitespace();
                let key = split.next().expect(
                    "due to the line not being empty, there should always be at least a key",
                );

                for c in key.chars() {
                    if !c.is_ascii_uppercase() && !c.is_ascii_digit() && c != '_' {
                        return Err(DataError::InvalidKeyFormat(key.to_string()));
                    }
                }

                let value = split
                    .next()
                    .ok_or(DataError::MissingValue(key.to_string()))?;

                program = program.replace(key, value);
            }
        }

        program.shrink_to_fit();

        Ok(program)
    } else {
        Ok(s.to_string())
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
        let input = ["pshb NUMBER_1", "stop", "", "data:", "    NUMBER_1 1"].join("\n");
        let expected = ["pshb 1", "stop"].join("\n");

        let actual = expand_data_section(&input)?;

        assert_eq!(actual, expected);

        Ok(())
    }
}
