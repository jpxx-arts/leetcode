pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(index) => index as i32,
        None => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "sadbutsad";
        let needle = "sad";

        assert_eq!(0, str_str(haystack.to_string(), needle.to_string()));
    }
}
