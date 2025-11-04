pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 0;

    for i in 0..nums.len() {
        if nums[k] != nums[i] {
            k += 1;

            nums[k] = nums[i];
        }
    }

    (k + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k() {
        let mut sorted_vec = vec![1, 1, 1, 1, 2, 3, 4, 4, 4];
        let k = remove_duplicates(&mut sorted_vec);

        assert_eq!(4, k);
    }

    #[test]
    fn test_new_vec() {
        let mut sorted_vec = vec![1, 1, 1, 1, 2, 3, 4, 4, 4];
        let k = remove_duplicates(&mut sorted_vec);

        assert_eq!(&[1, 2, 3, 4], &sorted_vec[..k as usize]);
    }
}
