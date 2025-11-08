pub mod test;

pub fn urlify(string: &mut Vec<char>, true_length: usize) -> &Vec<char>
{
    if string.is_empty()
    {
        return string;
    }

    let mut edit_index = string.len() - 1;
    let mut read_index = true_length - 1;

    loop
    {
        let ch = string[read_index];
        match ch
        {
            ' ' =>
            {
                string[edit_index] = '0';
                edit_index -= 1;
                string[edit_index] = '2';
                edit_index -= 1;
                string[edit_index] = '%';
            }
            _ =>
            {
                string[edit_index] = ch;
            }
        }

        if read_index == 0
        {
            break;
        }

        read_index -= 1;
        edit_index -= 1;
    }

    string
}

pub fn urlify_kevin(string: &mut Vec<char>, true_length: usize) -> &Vec<char> { urlify(string, true_length) }

fn shift_and_insert(string: &mut Vec<char>, index: usize) -> &Vec<char>
{
    let end_index = string.len() - 1;

    for i in 0..(string.len() - 2)
    {
        let curr_index = end_index - i;
        let swap_index = curr_index - 2;

        if curr_index == index
        {
            break;
        }

        string[curr_index] = string[swap_index];
    }

    string[index] = '%';
    string[index + 1] = '2';
    string[index + 2] = '0';

    string
}

pub fn urlify_tanner(string: &mut Vec<char>, _true_length: usize) -> &Vec<char>
{
    for i in 0..string.len()
    {
        let character = string[i];

        if character.is_whitespace()
        {
            shift_and_insert(string, i);
        }
    }

    string
}

pub fn urlify_mike(string: &mut Vec<char>, true_length: usize) -> &Vec<char>
{
    if string.is_empty()
    {
        return string;
    }
    let mut space_count = 0;
    let mut new_length = 0;
    let mut index = 0;

    loop
    {
        if index >= true_length
        {
            break;
        }
        if string[index].is_whitespace()
        {
            space_count += 1;
        }
        index += 1;
    }

    if space_count == 0
    {
        return string;
    }

    new_length = true_length + (space_count * 2);
    string[new_length - 1] = '\0';

    let mut index = true_length - 1;

    loop
    {
        let ch = string[index];
        if ch.is_whitespace()
        {
            string[new_length - 1] = '0';
            string[new_length - 2] = '2';
            string[new_length - 3] = '%';
            new_length -= 3;
        }
        else
        {
            string[new_length - 1] = string[index];
            new_length -= 1;
        }

        if index == 0
        {
            break;
        }
        index -= 1;
    }

    string
}

#[cfg(test)]
pub mod unit_test
{
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_urlify()
    {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases
        {
            let mut input_string = test_case.input_string.clone();
            let actual = urlify(&mut input_string, test_case.input_true_length);
            assert_eq!(*actual,
                       test_case.answer,
                       "input: '{},{}', expected: {}, actual: {}",
                       String::from_iter(test_case.input_string.clone()),
                       test_case.input_true_length,
                       String::from_iter(test_case.answer.clone()),
                       String::from_iter(actual.clone()));
        }
    }
}
