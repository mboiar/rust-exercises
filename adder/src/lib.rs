pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(val: usize) -> usize{
    val + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn always_fails() {
        panic!("Panic!!!");
    }
}
