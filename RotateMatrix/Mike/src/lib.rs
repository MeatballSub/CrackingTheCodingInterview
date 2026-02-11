pub mod test;

// Given an image represented by an NxN matrix, whee each pixel in the image is
// represented by an integer, write a method to rotate the image by 90 degrees.
// Do it in-place.
// Assume clockwise rotation: ↻
// ↱→↴
// ↑↻↓
// ⬑←↲

// fn rotated_position(row: usize, col: usize, n: usize) -> (usize, usize) {
//     (col, n - 1 - row)
// }

pub fn rotate_matrix(matrix: &mut [Vec<usize>]) -> &[Vec<usize>] {
    let n = matrix.len();
    if n <= 1 {
        return matrix;
    }

    for layer in 0..(n / 2) {
        let first = layer;
        let last = n - 1 - layer;

        for i in first..last {

            let offset = i - first;
            let mut left = matrix[last - offset][first];

            matrix[last - offset][first] = matrix[last][last - offset];;
            matrix[last][last - offset] = matrix[i][last];;
            matrix[i][last] = matrix[first][i];;
            matrix[first][i] = left;

        }
    }

    matrix
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_rotate_matrix() {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases {
            let original = test_case.matrix.clone();
            let actual = rotate_matrix(&mut test_case.matrix);
            assert_eq!(
                actual, test_case.answer,
                "input: '{:?}', expected: {:?}, actual: {:?}",
                original, test_case.answer, actual
            );
        }
    }
}
