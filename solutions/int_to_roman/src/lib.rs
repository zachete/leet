use std::collections::VecDeque;

pub fn int_to_roman(num: i32) -> String {
    let mut num_map = VecDeque::from([
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ]);

    let mut mut_num = num;
    let mut result = String::from("");

    while !num_map.is_empty() {
        let (symbol, number) = num_map.front().unwrap();
        let int = mut_num / number;

        if int > 0 {
            result += &symbol.to_string();
            mut_num -= number;
        } else {
            num_map.pop_front();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = int_to_roman(3749);
        assert_eq!(result, "MMMDCCXLIX");
    }

    #[test]
    fn case2() {
        let result = int_to_roman(58);
        assert_eq!(result, "LVIII");
    }

    #[test]
    fn case3() {
        let result = int_to_roman(1994);
        assert_eq!(result, "MCMXCIV");
    }
}
