fn symbol_to_digit(symbol: char) -> i32 {
    match symbol {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let mut result_number: i32 = 0;
    let mut chars_iter = s.chars().peekable();

    while let Some(symbol) = chars_iter.next() {
        let current_digit = symbol_to_digit(symbol);
        let next_symbol = chars_iter.peek();
        let next_digit = match next_symbol {
            Some(&value) => symbol_to_digit(value),
            None => 0,
        };

        if current_digit < next_digit {
            result_number += next_digit - current_digit;
            chars_iter.next();
        } else {
            result_number += current_digit;
        }
    }

    result_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = String::from("III");
        let result = roman_to_int(input);

        assert_eq!(result, 3);
    }

    #[test]
    fn case2() {
        let input = String::from("IV");
        let result = roman_to_int(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn case3() {
        let input = String::from("LVIII");
        let result = roman_to_int(input);

        assert_eq!(result, 58);
    }

    #[test]
    fn case4() {
        let input = String::from("MMMCMXC");
        let result = roman_to_int(input);

        assert_eq!(result, 3990)
    }
}
