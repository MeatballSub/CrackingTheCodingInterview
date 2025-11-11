pub mod test;

// Given an image represented by an NxN matrix, whee each pixel in the image is
// represented by an integer, write a method to rotate the image by 90 degrees.
// Do it in-place.
// Assume clockwise rotation: ↻
// ↱→↴
// ↑↻↓
// ⬑←↲
pub fn rotate_matrix(matrix: &[Vec<usize>]) -> &[Vec<usize>] {
    todo!("Implement this function")
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_rotate_matrix() {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases {
            let actual = rotate_matrix(&test_case.matrix);
            assert_eq!(
                actual, test_case.answer,
                "input: '{:?}', expected: {:?}, actual: {:?}",
                test_case.matrix, test_case.answer, actual
            );
        }
    }
}
