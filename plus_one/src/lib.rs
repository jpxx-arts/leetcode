pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut new_digits = digits.clone();

    let mut carry = 1;
    for ind_digit in (0..new_digits.len()).rev() {
        let digit = new_digits[ind_digit];

        if digit == 9 && carry == 1 {
            new_digits[ind_digit] = 0;
        } else {
            new_digits[ind_digit] += carry;
            carry = 0;
        }
    }

    if carry == 1 {
        new_digits.insert(0, 1);
    }

    new_digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_carry() {
        let digits = vec![1, 2, 3];
        let result = plus_one(digits);
        assert_eq!(result, vec![1, 2, 4]);
    }

    #[test]
    fn with_carry() {
        let digits = vec![1, 2, 9];
        let result = plus_one(digits);
        assert_eq!(result, vec![1, 3, 0]);
    }

    #[test]
    fn all_nines() {
        let digits = vec![9, 9, 9];
        let result = plus_one(digits);
        assert_eq!(result, vec![1, 0, 0, 0]);
    }
}
