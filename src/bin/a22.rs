// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 10, 20), 10, "Should be clamped to lower limit");
        assert_eq!(clamp(25, 10, 20), 20, "Should be clamped to upper limit");
        assert_eq!(clamp(15, 10, 20), 15, "Should remain unchanged");
    }

    #[test]
    fn test_div() {
        assert_eq!(div(10, 2), Some(5), "Should divide numbers");
        assert_eq!(div(10, 0), None, "Should return None when dividing by zero");
        assert_eq!(div(10, 3), Some(3), "Should perform integer division");
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat("a", "b"), "ab", "Should concatenate strings");
        assert_eq!(
            concat("hello", "world"),
            "helloworld",
            "Should concatenate strings"
        );
    }
}
