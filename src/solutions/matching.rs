pub enum Operations {
    Add,
    Sub,
    Mul,
    Dev,
}

pub fn match_operations(a: i16, b: i16, operation: Operations) -> i16 {
    match operation {
        Operations::Add => a + b,
        Operations::Dev => a / b,
        Operations::Mul => a * b,
        Operations::Sub => a - b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(match_operations(20, 10, Operations::Add), 30)
    }

    #[test]
    fn test_sub() {
        assert_eq!(match_operations(20, 10, Operations::Sub), 10)
    }

    #[test]
    fn test_mul() {
        assert_eq!(match_operations(20, 10, Operations::Mul), 200)
    }

    #[test]
    fn test_dev() {
        assert_eq!(match_operations(20, 10, Operations::Dev), 2)
    }
}
