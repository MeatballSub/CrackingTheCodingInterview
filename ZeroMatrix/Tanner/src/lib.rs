pub mod test;

// Write an algorithm such that if an element in an N x N matrix is zero(0),
// its entire row and column are set to zero(0)
pub fn zero_matrix(matrix: &mut [Vec<usize>]) -> &[Vec<usize>] {
    todo!("Implement this function")
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_zero_matrix() {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases {
            let original = test_case.matrix.clone();
            let actual = zero_matrix(&mut test_case.matrix);
            assert_eq!(
                actual, test_case.answer,
                "input: '{:?}', expected: {:?}, actual: {:?}",
                original, test_case.answer, actual
            );
        }
    }
}
