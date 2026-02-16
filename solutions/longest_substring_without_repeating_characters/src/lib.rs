use std::{cmp, collections::HashMap};

pub fn length_of_longest_substring(input: String) -> i32 {
    let mut char_positions: HashMap<char, usize> = HashMap::with_capacity(input.len());
    let mut max_length = 0;
    let mut window_start = 0;

    for (current_index, current_char) in input.char_indices() {
        if let Some(&last_position) = char_positions.get(&current_char) {
            if last_position >= window_start {
                window_start = last_position + 1;
            }
        }

        char_positions.insert(current_char, current_index);
        let current_length = current_index - window_start + 1;
        max_length = cmp::max(max_length, current_length)
    }

    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = String::from("abcabcbb");
        let result = length_of_longest_substring(input);

        assert_eq!(result, 3);
    }

    #[test]
    fn example2() {
        let input = String::from("bbbbb");
        let result = length_of_longest_substring(input);

        assert_eq!(result, 1);
    }

    #[test]
    fn example3() {
        let input = String::from("pwwkew");
        let result = length_of_longest_substring(input);

        assert_eq!(result, 3);
    }
}
