pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut j = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums[j] = nums[i];
            j += 1;
        }
    }

    j as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;

        let k = remove_element(&mut nums, val);

        assert_eq!(3, k);
    }
}
