use std::{fmt::Display, str::FromStr};

/// Validates that the given value is within the given range and returns it.
pub fn validate_and_extract<T>(val: T, low: T, high: T, param_name: &str) -> Result<T, String>
where
    T: PartialOrd + Display + Copy,
{
    if (low..=high).contains(&val) {
        Ok(val)
    } else {
        Err(format!(
            "Invalid value for {}: {} is outside the range [{}, {}]",
            param_name, val, low, high
        ))
    }
}

/// Validates that the given value within the option is within the given range and returns the option.
pub fn validate_and_extract_option<T>(
    val: Option<T>,
    low: T,
    high: T,
    param_name: &str,
) -> Result<Option<T>, String>
where
    T: PartialOrd + Display + Copy,
{
    match val {
        Some(v) => Ok(Some(validate_and_extract(v, low, high, param_name)?)),
        None => Ok(None),
    }
}

/// Parses a comma-separated list of values into a vector of the given type.
pub fn parse_comma_separated<T>(val: &str, param_name: &str) -> Result<Vec<T>, String>
where
    T: FromStr + Display,
    T::Err: Display,
{
    val.split(',')
        .map(|s| {
            s.trim()
                .parse::<T>()
                .map_err(|err| format!("Problem parsing value '{}' for {}: {}", s, param_name, err))
        })
        .collect()
}

/// Parses a comma-separated list (if present) into a vector of the given type.
pub fn parse_comma_separated_option<T>(
    val: Option<String>,
    param_name: &str,
) -> Result<Option<Vec<T>>, String>
where
    T: FromStr + Display,
    T::Err: Display,
{
    match val {
        Some(s) => Ok(Some(parse_comma_separated(&s, param_name)?)),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(50, 30, 100, "bpm", Ok(50))]
    #[case(30, 30, 100, "bpm", Ok(30))]
    #[case(100, 30, 100, "bpm", Ok(100))]
    #[case(
        20,
        30,
        100,
        "bpm",
        Err("Invalid value for bpm: 20 is outside the range [30, 100]")
    )]
    #[case(
        150,
        30,
        100,
        "bpm",
        Err("Invalid value for bpm: 150 is outside the range [30, 100]")
    )]
    fn test_validate_and_extract(
        #[case] val: u32,
        #[case] low: u32,
        #[case] high: u32,
        #[case] param_name: &str,
        #[case] expected: Result<u32, &str>,
    ) {
        let result = validate_and_extract(val, low, high, param_name);

        match expected {
            Ok(expected_val) => {
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), expected_val);
            }
            Err(expected_err) => {
                assert!(result.is_err());
                let err_msg = result.unwrap_err();
                assert!(err_msg.contains(expected_err));
            }
        }
    }

    #[rstest]
    #[case(Some(50), 30, 100, "bpm", Ok(Some(50)))]
    #[case(Some(30), 30, 100, "bpm", Ok(Some(30)))]
    #[case(Some(100), 30, 100, "bpm", Ok(Some(100)))]
    #[case(
        Some(20),
        30,
        100,
        "bpm",
        Err("Invalid value for bpm: 20 is outside the range [30, 100]")
    )]
    #[case(
        Some(150),
        30,
        100,
        "bpm",
        Err("Invalid value for bpm: 150 is outside the range [30, 100]")
    )]
    #[case(None, 30, 100, "bpm", Ok(None))]
    fn test_validate_and_extract_option(
        #[case] val: Option<u32>,
        #[case] low: u32,
        #[case] high: u32,
        #[case] param_name: &str,
        #[case] expected: Result<Option<u32>, &str>,
    ) {
        let result = validate_and_extract_option(val, low, high, param_name);

        match expected {
            Ok(expected_val) => {
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), expected_val);
            }
            Err(expected_err) => {
                assert!(result.is_err());
                let err_msg = result.unwrap_err();
                assert!(err_msg.contains(expected_err));
            }
        }
    }

    #[rstest]
    #[case("1,2,3", "numbers", Ok(vec![1, 2, 3]))]
    #[case("1,2,abc", "numbers", Err("Problem parsing value 'abc' for numbers"))]
    fn test_parse_comma_separated_nums(
        #[case] val: &str,
        #[case] param_name: &str,
        #[case] expected: Result<Vec<u32>, &str>,
    ) {
        let result: Result<Vec<u32>, String> = parse_comma_separated(val, param_name);

        match expected {
            Ok(expected_val) => {
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), expected_val);
            }
            Err(expected_err) => {
                assert!(result.is_err());
                let err_msg = result.unwrap_err();
                assert!(err_msg.contains(expected_err));
            }
        }
    }

    #[rstest]
    #[case("one,two,three", "numbers", Ok(vec!["one", "two", "three"]))]
    fn test_parse_comma_separated_strings(
        #[case] val: &str,
        #[case] param_name: &str,
        #[case] expected: Result<Vec<&str>, &str>,
    ) {
        let result: Result<Vec<String>, String> = parse_comma_separated(val, param_name);

        match expected {
            Ok(expected_val) => {
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), expected_val);
            }
            Err(expected_err) => {
                assert!(result.is_err());
                let err_msg = result.unwrap_err();
                assert!(err_msg.contains(expected_err));
            }
        }
    }

    #[rstest]
    #[case(Some("1,2,3".to_string()), "numbers", Ok(Some(vec![1, 2, 3])))]
    #[case(Some("1,2,abc".to_string()), "numbers", Err("Problem parsing value 'abc' for numbers"))]
    #[case(None, "numbers", Ok(None))]
    fn test_parse_comma_separated_num_option(
        #[case] val: Option<String>,
        #[case] param_name: &str,
        #[case] expected: Result<Option<Vec<u32>>, &str>,
    ) {
        let result = parse_comma_separated_option(val, param_name);

        match expected {
            Ok(expected_val) => {
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), expected_val);
            }
            Err(expected_err) => {
                assert!(result.is_err());
                let err_msg = result.unwrap_err();
                assert!(err_msg.contains(expected_err));
            }
        }
    }
}
