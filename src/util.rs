pub fn remove_whitespace(s: String) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(test)]
mod util {
    use crate::util::remove_whitespace;

    #[test]
    fn remove_whitespace_test() {
        let s = String::from(" r e t c o n ");
        let removed_whitespace = remove_whitespace(s);
        assert_eq!(removed_whitespace, "retcon");
    }
}
