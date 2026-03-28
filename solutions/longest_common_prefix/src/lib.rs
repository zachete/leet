pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let len = strs.len();
    let mut common_prefix = String::from("");

    if len == 0 {
        return common_prefix;
    }

    let mut cursor = 0;

    'main: loop {
        let mut prefix = String::from("");

        for i in 0..len {
            let str = &strs[i];
            let char = str.chars().nth(cursor);

            match char {
                Some(value) => {
                    if prefix == "" {
                        prefix = String::from(value);
                    } else if prefix != value.to_string() {
                        break 'main;
                    }
                }
                None => break 'main,
            };
        }

        common_prefix += &prefix;
        cursor += 1;
    }

    common_prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let result = longest_common_prefix(strs);
        assert_eq!(result, "fl");
    }

    #[test]
    fn case2() {
        let strs = vec!["ab".to_string(), "a".to_string()];
        let result = longest_common_prefix(strs);
        assert_eq!(result, "a");
    }

    #[test]
    fn case3() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let result = longest_common_prefix(strs);
        assert_eq!(result, "");
    }

    #[test]
    fn case4() {
        let strs = vec![
            "flower".to_string(),
            "flower".to_string(),
            "flower".to_string(),
            "flower".to_string(),
        ];
        let result = longest_common_prefix(strs);
        assert_eq!(result, "flower");
    }
}
