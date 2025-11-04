/*
Input: nums = [1,3,5,6], target = 5
Output: 2

Input: nums = [1,3,5,6], target = 2
Output: 1
*/

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let ind = nums.len()/2;
    let val = nums[ind];

    if val == target {
        return ind as i32;
    }

    if nums.len() == 1 {
        if target < val {
            return ind as i32;
        } else {
            return ind as i32 + 1;
        }
    }

    if val > target {
        return search_insert(nums[0..ind].to_vec(), target);
    }

    if val < target {
        return search_insert(nums[ind..nums.len()].to_vec(), target) + ind as i32;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_existent_ind() {
        let nums = vec![1,3,5,6];

        assert_eq!(3, search_insert(nums, 6));
    }

    #[test]
    fn ind_not_found() {
        let nums = vec![1,3,5,6];

        println!("result {}", search_insert(nums.clone(), 4));
        assert_eq!(2, search_insert(nums, 4));
    }
}
