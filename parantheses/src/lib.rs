const OPEN_SYMBOLS: [char; 3] = ['(', '[', '{'];

fn corresponding_open(l: char) -> Option<char> {
    match l {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        _ => None,
    }
}

pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for c in s.chars() {
        if OPEN_SYMBOLS.contains(&c) {
            stack.push(c);
        } else if let Some(expected) = corresponding_open(c) {
            if stack.pop() != Some(expected) {
                return false;
            }
        } else {
            return false;
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(is_valid("[({})]".to_string()));
    }
}
