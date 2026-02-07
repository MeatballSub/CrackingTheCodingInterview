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

fn zero_rows(matrix: &mut [Vec<usize>], rows: Vec<bool>) -> &[Vec<usize>] {
    for index in 0..rows.len() {
        if rows[index] {
            zero_row(matrix, index);
        }
    }
    matrix
}

fn zero_col(matrix: &mut [Vec<usize>], j: usize) -> &[Vec<usize>] {
    for i in 0..matrix.len() {
        matrix[i][j] = 0
    }

    matrix
}

fn zero_cols(matrix: &mut [Vec<usize>], cols: Vec<bool>) -> &[Vec<usize>] {
    for index in 0..cols.len() {
        if cols[index] {
            zero_col(matrix, index);
        }
    }
    matrix
}

// Write an algorithm such that if an element in an N x N matrix is zero(0),
// its entire row and column are set to zero(0)
pub fn zero_matrix(matrix: &mut [Vec<usize>]) -> &[Vec<usize>] {
    let num_rows: usize = matrix.len();
    let num_cols = matrix[0].len();

    let mut rows = vec![false; num_rows];
    let mut cols = vec![false; num_cols];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                rows[i] = true;
                cols[j] = true;
            }
        }
    }

    zero_rows(matrix, rows);
    zero_cols(matrix, cols);

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
