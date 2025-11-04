fn symbol_to_int(sym_prev: u8, sym_next: u8) -> (usize, i32) {
    match sym_prev {
        b'I' => (0, 1),

        b'V' => {
            if sym_next == b'I' {
                (1, 4)
            } else {
                (0, 5)
            }
        }

        b'X' => {
            if sym_next == b'I' {
                (1, 9)
            } else {
                (0, 10)
            }
        }

        b'L' => {
            if sym_next == b'X' {
                (1, 40)
            } else {
                (0, 50)
            }
        }

        b'C' => {
            if sym_next == b'X' {
                (1, 90)
            } else {
                (0, 100)
            }
        }

        b'D' => {
            if sym_next == b'C' {
                (1, 400)
            } else {
                (0, 500)
            }
        }

        b'M' => {
            if sym_next == b'C' {
                (1, 900)
            } else {
                (0, 1000)
            }
        }

        _ => (0, 0),
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let mut amount = 0;
    let mut s_bytes = s.into_bytes();
    s_bytes.reverse();

    let mut i = 0;
    while i < s_bytes.len() {
        let sym_prev = s_bytes[i];

        let sym_next = if i < s_bytes.len() - 1 {
            s_bytes[i + 1]
        } else {
            0
        };

        let (i_add, amount_sum) = symbol_to_int(sym_prev, sym_next);
        i += i_add + 1;
        amount += amount_sum;
    }

    amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_to_int_without_sub() {
        let inp = String::from("LVIII");
        assert_eq!(58, roman_to_int(inp));
    }

    #[test]
    fn roman_to_int_with_sub() {
        let inp = String::from("MCMXCIV");
        assert_eq!(1994, roman_to_int(inp));
    }
}
