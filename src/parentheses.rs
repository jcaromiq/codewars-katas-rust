//https://www.codewars.com/kata/5f7c38eb54307c002a2b8cc8/train/rust

fn remove_parentheses(s: &str) -> String {
    let mut p = 0;
    s.chars()
        .filter(|c| match c {
            '(' => { p += 1; false },
            ')' => { p -= 1; false },
            _ => p == 0,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::remove_parentheses;

    #[test]
    fn sample_tests() {
        assert_eq!(remove_parentheses("example(unwanted thing)example"), "exampleexample");
        assert_eq!(remove_parentheses("example (unwanted thing) example"), "example  example");
        assert_eq!(remove_parentheses("a (bc d)e"), "a e");
        assert_eq!(remove_parentheses("a(b(c))"), "a");
        assert_eq!(remove_parentheses("hello example (words(more words) here) something"), "hello example  something");
        assert_eq!(remove_parentheses("(first group) (second group) (third group)"), "  ");
    }
}
