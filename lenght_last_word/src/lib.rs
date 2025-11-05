/*
Given a string s consisting of words and spaces, return the length of the last word in the string.
"   fly me   to   the moon  "
*/

pub fn length_of_last_word(s: String) -> i32 {
    let mut last_word_len = 0;
    let mut found_last_word = false;

    for letter_byte in s.bytes().rev() {
        if letter_byte != b' ' {
            found_last_word = true;
            last_word_len += 1;
        } else if found_last_word {
            break;
        }
    }

    last_word_len
}

/*
pub fn length_of_last_word(s: String) -> i32 {
if s.is_empty() {
    return 0;
}

s.trim_end().split_whitespace().last().unwrap().len() as i32
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let result = String::from("Hello World");
        assert_eq!(5, length_of_last_word(result));
    }

    #[test]
    fn with_additional_whitespaces() {
        let result = String::from("    fly me to the moon    ");
        assert_eq!(4, length_of_last_word(result));
    }
}
