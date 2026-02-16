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

pub fn zero_matrix_kevin_2(matrix: &mut [Vec<usize>]) -> &[Vec<usize>]
{
    if !matrix.is_empty()
    {
        let n = matrix.len();
        let mut rows = 0;
        let mut cols = 0;

        for row in 0..n
        {
            for col in 0..n
            {
                if matrix[row][col] == 0
                {
                    rows |= 1 << row;
                    cols |= 1 << col;
                }
            }
        }

        for i in 0..n
        {
            if (rows & (1 << i)) != 0
            {
                for col in 0..n
                {
                    matrix[i][col] = 0;
                }
            }

            if (cols & (1 << i)) != 0
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

fn zero_row(matrix: &mut [Vec<usize>], i: usize) -> &[Vec<usize>]
{
    for j in 0..matrix[i].len()
    {
        matrix[i][j] = 0
    }
    matrix
}

fn zero_rows(matrix: &mut [Vec<usize>], rows: Vec<bool>) -> &[Vec<usize>]
{
    for index in 0..rows.len()
    {
        if rows[index]
        {
            zero_row(matrix, index);
        }
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

fn zero_cols(matrix: &mut [Vec<usize>], cols: Vec<bool>) -> &[Vec<usize>]
{
    for index in 0..cols.len()
    {
        if cols[index]
        {
            zero_col(matrix, index);
        }
    }
    matrix
}

// Write an algorithm such that if an element in an N x N matrix is zero(0),
// its entire row and column are set to zero(0)
pub fn zero_matrix_tanner(matrix: &mut [Vec<usize>]) -> &[Vec<usize>]
{
    let num_rows: usize = matrix.len();
    let num_cols = matrix[0].len();

    let mut rows = vec![false; num_rows];
    let mut cols = vec![false; num_cols];

    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            if matrix[i][j] == 0
            {
                rows[i] = true;
                cols[j] = true;
            }
        }
    }

    zero_rows(matrix, rows);
    zero_cols(matrix, cols);

    matrix
}

pub fn zero_matrix_mike(matrix: &mut [Vec<usize>]) -> &[Vec<usize>]
{
    let n = matrix.len();
    if n <= 1
    {
        return matrix;
    }

    let mut rows_with_zeroes: Vec<i32> = Vec::new();
    let mut cols_with_zeroes: Vec<i32> = Vec::new();

    matrix.iter().enumerate().for_each(|(r_index, row)| {
                                 row.iter().enumerate().for_each(|(c_index, column)| {
                                                           if *column == 0
                                                           {
                                                               rows_with_zeroes.push(r_index.try_into().unwrap());
                                                               cols_with_zeroes.push(c_index.try_into().unwrap());
                                                           }
                                                       });
                             });

    if cols_with_zeroes.is_empty() && rows_with_zeroes.is_empty()
    {
        return matrix;
    }

    for r_index in &rows_with_zeroes
    {
        let final_r_index = *r_index as usize;
        matrix[final_r_index].fill(0);
    }

    for c_index in &cols_with_zeroes
    {
        let final_c_index = *c_index as usize;
        for row in matrix.iter_mut()
        {
            row[final_c_index] = 0;
        }
    }

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
            let actual = zero_matrix_kevin_2(&mut test_case.matrix);
            assert_eq!(actual, test_case.answer, "input: '{:?}', expected: {:?}, actual: {:?}", original, test_case.answer, actual);
        }
    }
}
