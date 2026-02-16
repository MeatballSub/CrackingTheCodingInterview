pub mod test;

// Write an algorithm such that if an element in an N x N matrix is zero(0),
// its entire row and column are set to zero(0)
pub fn zero_matrix(matrix: &mut [Vec<usize>]) -> &[Vec<usize>] {

    let n = matrix.len();
    if n <= 1 {
        return matrix;
    }

    let mut rows_with_zeroes: Vec<i32> =  Vec::new();
    let mut cols_with_zeroes: Vec<i32> =  Vec::new();

    matrix.iter().enumerate().for_each(|(r_index,row)| {
        row.iter().enumerate().for_each(|(c_index, column)| {

            if *column == 0 {
                rows_with_zeroes.push(r_index.try_into().unwrap());
                cols_with_zeroes.push(c_index.try_into().unwrap());
            }
        });
    });

    if cols_with_zeroes.is_empty() && rows_with_zeroes.is_empty() {
        return matrix;
    }

    for r_index in &rows_with_zeroes {
        let final_r_index = *r_index as usize;
        matrix[final_r_index].fill(0);
    }

    for c_index in &cols_with_zeroes {
        let final_c_index = *c_index as usize;
        for row in matrix.iter_mut() {
            row[final_c_index] = 0;
        }
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
