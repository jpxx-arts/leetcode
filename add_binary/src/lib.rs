/*
0 0 0 = 0, 0
0 0 1 = 1, 0
0 1 0 = 1, 0
0 1 1 = 0, 1
1 0 0 = 1, 0
1 0 1 = 0, 1
1 1 0 = 0, 1
1 1 1 = 1, 1

sum = a'b'c + a'bc' + ab'c' + abc
carry = a'bc + ab'c + abc' + abc
*/

pub fn full_adding(a: bool, b: bool, c: bool) -> (char, bool) {
    (if a ^ b ^ c { '1' } else { '0' }, (a & b) | c & (a ^ b))
}

pub fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();

    let greater_str_len;
    if a.len() >= b.len() {
        greater_str_len = a.len();
    } else {
        greater_str_len = b.len();
    }

    let mut a_bits = a.bytes().rev();
    let mut b_bits = b.bytes().rev();

    let mut current_carry: bool = false;
    let mut actual_bit;
    for _ in 0..greater_str_len {
        let a_bit: bool = a_bits.next().map(|c| c == b'1').unwrap_or(false);
        let b_bit: bool = b_bits.next().map(|c| c == b'1').unwrap_or(false);

        (actual_bit, current_carry) = full_adding(a_bit, b_bit, current_carry);

        result.push(actual_bit);
    }

    if current_carry {
        result.push('1');
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = String::from("1010");
        let mut b = String::from("1011");

        assert_eq!("10101".to_string(), add_binary(a, b));

        a = String::from("11");
        b = String::from("1");

        assert_eq!("100".to_string(), add_binary(a, b));
    }
}
