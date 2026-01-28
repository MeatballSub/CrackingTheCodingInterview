use std::collections::HashSet;

pub mod test;

fn _print_matrix(matrix: &mut [Vec<usize>]) {
    for i in 0..matrix.len() {
        println!("{:?}", matrix[i]);
    }
    println!()
}

fn zero_row(matrix: &mut [Vec<usize>], i: usize) -> &[Vec<usize>] {
    for j in 0..matrix[i].len() {
        matrix[i][j] = 0
    }

    matrix
}

fn zero_col(matrix: &mut [Vec<usize>], j: usize) -> &[Vec<usize>] {
    for i in 0..matrix.len() {
        matrix[i][j] = 0
    }

    matrix
}

// Write an algorithm such that if an element in an N x N matrix is zero(0),
// its entire row and column are set to zero(0)
pub fn zero_matrix(matrix: &mut [Vec<usize>]) -> &[Vec<usize>] {
    let mut rows = HashSet::<usize>::new();
    let mut cols = HashSet::<usize>::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                rows.insert(i);
                cols.insert(j);
            }
        }
    }

    for row in rows {
        zero_row(matrix, row);
    }

    for col in cols {
        zero_col(matrix, col);
    }

    matrix
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
