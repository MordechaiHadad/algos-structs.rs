pub fn linear_search(haystack: &[i32], needle: i32) -> bool {
    for n in 0..haystack.len() {

        if haystack[n] == needle {
            return true;
        }
    }
    false
}
