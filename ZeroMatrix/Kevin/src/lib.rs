use std::collections::HashSet;

pub mod test;

// Write an algorithm such that if an element in an N x N matrix is zero(0),
// its entire row and column are set to zero(0)
pub fn zero_matrix_kevin(matrix: &mut [Vec<usize>]) -> &[Vec<usize>]
{
    if !matrix.is_empty()
    {
        let n = matrix.len();
        let mut rows = vec![false; n];
        let mut cols = vec![false; n];

        for row in 0..n
        {
            for col in 0..n
            {
                let is_zero = matrix[row][col] == 0;
                rows[row] |= is_zero;
                cols[col] |= is_zero;
            }
        }

        for i in 0..n
        {
            if rows[i]
            {
                for col in 0..n
                {
                    matrix[i][col] = 0;
                }
            }

            if cols[i]
            {
                for row in 0..n
                {
                    matrix[row][i] = 0;
                }
            }
        }
    }

    matrix
}

fn _print_matrix(matrix: &mut [Vec<usize>])
{
    for i in 0..matrix.len()
    {
        println!("{:?}", matrix[i]);
    }
    println!()
}

fn zero_row(matrix: &mut [Vec<usize>], i: usize) -> &[Vec<usize>]
{
    for j in 0..matrix[i].len()
    {
        matrix[i][j] = 0
    }
    matrix
}

fn zero_rows(matrix: &mut [Vec<usize>], rows: HashSet<usize>) -> &[Vec<usize>]
{
    for row in rows
    {
        zero_row(matrix, row);
    }
    matrix
}

fn zero_col(matrix: &mut [Vec<usize>], j: usize) -> &[Vec<usize>]
{
    for i in 0..matrix.len()
    {
        matrix[i][j] = 0
    }

    matrix
}

fn zero_cols(matrix: &mut [Vec<usize>], cols: HashSet<usize>) -> &[Vec<usize>]
{
    for col in cols
    {
        zero_col(matrix, col);
    }
    matrix
}

// Write an algorithm such that if an element in an N x N matrix is zero(0),
// its entire row and column are set to zero(0)
pub fn zero_matrix_tanner(matrix: &mut [Vec<usize>]) -> &[Vec<usize>]
{
    let mut rows = HashSet::<usize>::new();
    let mut cols = HashSet::<usize>::new();

    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            if matrix[i][j] == 0
            {
                rows.insert(i);
                cols.insert(j);
            }
        }
    }

    zero_rows(matrix, rows);
    zero_cols(matrix, cols);

    matrix
}

#[cfg(test)]
pub mod unit_test
{
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_zero_matrix()
    {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases
        {
            let original = test_case.matrix.clone();
            let actual = zero_matrix_kevin(&mut test_case.matrix);
            assert_eq!(actual, test_case.answer, "input: '{:?}', expected: {:?}, actual: {:?}", original, test_case.answer, actual);
        }
    }
}
