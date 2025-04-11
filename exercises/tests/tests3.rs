pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));  // 测试偶数返回 true
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5)); // 测试奇数返回 false
    }
}