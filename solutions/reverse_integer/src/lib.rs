pub fn reverse(x: i32) -> i32 {
    let is_negative = x < 0;
    let mut digits: Vec<i64> = Vec::new();
    let x_i64 = x as i64;
    let mut carry = x_i64.abs();

    loop {
        if carry == 0 {
            break;
        }

        let modulo = carry % 10;
        carry /= 10;
        digits.push(modulo);
    }

    let num_len = digits.len();
    let mut result = 0_i64;

    for (num, digit) in digits.iter().enumerate() {
        let multiplier = 10_i32.pow((num_len - num - 1) as u32);
        result += multiplier as i64 * digit;

        if result > i32::MAX as i64 {
            return 0;
        }
    }

    if is_negative {
        result *= -1;
    }

    result.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = reverse(123);

        assert_eq!(result, 321);
    }

    #[test]
    fn case2() {
        let result = reverse(-123);

        assert_eq!(result, -321);
    }

    #[test]
    fn case3() {
        let result = reverse(120);

        assert_eq!(result, 21);
    }

    #[test]
    fn case4() {
        let result = reverse(1534236469);

        assert_eq!(result, 0);
    }

    #[test]
    fn case5() {
        let result = reverse(-2147483648);

        assert_eq!(result, 0);
    }
}
