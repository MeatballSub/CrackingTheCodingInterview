pub mod test;

// Given an image represented by an NxN matrix, where each pixel in the image is
// represented by an integer, write a method to rotate the image by 90 degrees.
// Do it in-place.
// Assume clockwise rotation: ↻
// ↱→↴
// ↑↻↓
// ⬑←↲
pub fn rotate_matrix_kevin(matrix: &mut [Vec<usize>]) -> &[Vec<usize>]
{
    for y in 0..((matrix.len() + 1) / 2)
    {
        for x in y..(matrix[y].len() - y - 1)
        {
            let temp = *rotated_value(matrix, x, y, 0);
            *rotated_value(matrix, x, y, 0) = *rotated_value(matrix, x, y, 1);
            *rotated_value(matrix, x, y, 1) = *rotated_value(matrix, x, y, 2);
            *rotated_value(matrix, x, y, 2) = *rotated_value(matrix, x, y, 3);
            *rotated_value(matrix, x, y, 3) = temp;
        }
    }

    matrix
}

// Note: This recursive function could be unrolled(and probably eliminated) for
// performance gains
fn rotated_value(matrix: &mut [Vec<usize>], x: usize, y: usize, offset: usize) -> &mut usize
{
    if offset == 0
    {
        return &mut matrix[y][x];
    }

    rotated_value(matrix, y, matrix.len() - 1 - x, offset - 1)
}

fn rotate_spot(matrix: &mut [Vec<usize>], row: usize, col: usize) -> &[Vec<usize>]
{
    let temp = matrix[row][col];

    let mut curr_row = row;
    let mut curr_col = col;

    for _i in 0..3
    {
        let new_row = (matrix.len() - 1) - curr_col;
        let new_col = curr_row;

        matrix[curr_row][curr_col] = matrix[new_row][new_col];

        curr_row = new_row;
        curr_col = new_col;
    }

    matrix[curr_row][curr_col] = temp;

    matrix
}

// Given an image represented by an NxN matrix, whee each pixel in the image is
// represented by an integer, write a method to rotate the image by 90 degrees.
// Do it in-place.
// Assume clockwise rotation: ↻
// ↱→↴
// ↑↻↓
// ⬑←↲
pub fn rotate_matrix_tanner(matrix: &mut [Vec<usize>]) -> &[Vec<usize>]
{
    for i in 0..=(matrix.len() / 2)
    {
        let row = &matrix[i];
        let last_col = row.len() - 1 - i;
        for j in i..last_col
        {
            rotate_spot(matrix, i, j);
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
    fn test_rotate_matrix()
    {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases
        {
            let original = test_case.matrix.clone();
            let actual = rotate_matrix_kevin(&mut test_case.matrix);
            assert_eq!(actual, test_case.answer, "input: '{:?}', expected: {:?}, actual: {:?}", original, test_case.answer, actual);
        }
    }
}
