use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, u16> = HashMap::with_capacity(nums.len());

    for (index, &num) in nums.iter().enumerate() {
        let partner = target - num;

        if let Some(&partner_index) = seen.get(&partner) {
            return vec![partner_index as i32, index as i32];
        }

        seen.insert(num, index as u16);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(input, target);

        assert_eq!(result, vec![0, 1])
    }
}
