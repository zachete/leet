use std::i32;

pub fn my_atoi(s: String) -> i32 {
    let mut sign: Option<i64> = None;
    let mut digits = Vec::with_capacity(s.len());
    let mut number_started = false;
    let mut see_digit = false;

    for ch in s.chars() {
        match ch {
            ' ' => {
                if number_started || see_digit {
                    break;
                }
            }
            '-' => {
                if number_started || sign.is_some() || see_digit {
                    break;
                }

                sign = Some(-1);
                number_started = true;
            }
            '+' => {
                if number_started || sign.is_some() || see_digit {
                    break;
                }

                sign = Some(1);
                number_started = true;
            }
            val => {
                if !ch.is_ascii_digit() {
                    break;
                }

                see_digit = true;

                let digit = val.to_digit(10).unwrap();

                if digit == 0 && digits.len() == 0 {
                    continue;
                }

                digits.push(digit);
                number_started = true;
            }
        }
    }

    if digits.len() == 0 {
        return 0;
    }

    let mut result: i64 = 0;

    for (pos, &digit) in digits.iter().rev().enumerate() {
        let base_multiplier = 10_i64.pow(pos as u32);

        if digit == 0 {
            if result + base_multiplier as i64 > i32::MAX as i64 {
                result += base_multiplier;
                break;
            }

            continue;
        }

        result += digit as i64 * base_multiplier;

        if result > i32::MAX as i64 {
            break;
        }
    }

    let final_result: i64 = match sign {
        Some(val) => val * result as i64,
        None => result as i64,
    };

    final_result.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_positive_number() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test_negative_number_with_leading_whitespace() {
        assert_eq!(my_atoi(" -042".to_string()), -42);
    }

    #[test]
    fn test_leading_whitespace() {
        assert_eq!(my_atoi("   42".to_string()), 42);
        assert_eq!(my_atoi("  123".to_string()), 123);
    }

    #[test]
    fn test_positive_sign() {
        assert_eq!(my_atoi("+42".to_string()), 42);
        assert_eq!(my_atoi("  +123".to_string()), 123);
    }

    #[test]
    fn test_negative_sign() {
        assert_eq!(my_atoi("-42".to_string()), -42);
        assert_eq!(my_atoi("  -123".to_string()), -123);
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(my_atoi("000042".to_string()), 42);
        assert_eq!(my_atoi("-000123".to_string()), -123);
        assert_eq!(my_atoi("010".to_string()), 10);
        assert_eq!(my_atoi("+000456".to_string()), 456);
    }

    #[test]
    fn test_trailing_non_digit_characters() {
        assert_eq!(my_atoi("42abc".to_string()), 42);
        assert_eq!(my_atoi("123xyz789".to_string()), 123);
        assert_eq!(my_atoi("0-1".to_string()), 0);
        assert_eq!(my_atoi("-42hello".to_string()), -42);
    }

    #[test]
    fn test_no_digits() {
        assert_eq!(my_atoi("abc".to_string()), 0);
        assert_eq!(my_atoi("  ".to_string()), 0);
        assert_eq!(my_atoi("".to_string()), 0);
        assert_eq!(my_atoi("+-2".to_string()), 0);
        assert_eq!(my_atoi("-+2".to_string()), 0);
    }

    #[test]
    fn test_overflow_positive() {
        assert_eq!(my_atoi("2147483648".to_string()), 2147483647);
        assert_eq!(my_atoi("9999999999".to_string()), 2147483647);
        assert_eq!(my_atoi("9223372036854775808".to_string()), 2147483647);
        assert_eq!(my_atoi("18446744073709551617".to_string()), 2147483647);
        assert_eq!(my_atoi("10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000522545459".to_string()), 2147483647);
    }

    #[test]
    fn test_overflow_negative() {
        assert_eq!(my_atoi("-2147483649".to_string()), -2147483648);
        assert_eq!(my_atoi("-9999999999".to_string()), -2147483648);
        assert_eq!(my_atoi("-1010023630o4".to_string()), -1010023630);
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(my_atoi("2147483647".to_string()), 2147483647);
        assert_eq!(my_atoi("-2147483648".to_string()), -2147483648);
    }

    #[test]
    fn test_mixed_whitespace_and_signs() {
        assert_eq!(my_atoi("   +0   ".to_string()), 0);
        assert_eq!(my_atoi("   -0   ".to_string()), 0);
        assert_eq!(my_atoi(" +  413".to_string()), 0);
    }

    #[test]
    fn test_complex_cases() {
        assert_eq!(my_atoi("words and 987".to_string()), 0);
        assert_eq!(my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(my_atoi("  0000000000012345678".to_string()), 12345678);
    }
}
