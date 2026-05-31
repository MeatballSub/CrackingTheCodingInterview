mod linked_list;
pub mod test;

#[cfg(test)]
pub mod unit_test
{
    use crate::test::read_test_cases;

    #[test]
    fn test_partition_stable_safe()
    {
        let mut test_cases = read_test_cases();
        for ref mut test_case in &mut test_cases
        {
            println!("Test Case:");
            println!("Input: {:?} {{partition_value: {}}}", test_case.list, test_case.partition_value);
            test_case.list.partition_stable_safe(test_case.partition_value);
            let mut ge_found = false;
            for item in &test_case.list
            {
                let is_ge = *item >= test_case.partition_value;
                ge_found |= is_ge;
                let symbol = if is_ge { ">=" } else { "<" };
                println!("{} {} {}", item, symbol, test_case.partition_value);
                assert!(is_ge == ge_found,
                        "item less than the partition value found after values greater than or equal to the partition value");
            }
            println!("Ok");
            println!("Output: {:?}", test_case.list);
            println!();
        }
    }

    #[test]
    fn test_partition_stable_unsafe()
    {
        let mut test_cases = read_test_cases();
        for ref mut test_case in &mut test_cases
        {
            println!("Test Case:");
            println!("Input: {:?} {{partition_value: {}}}", test_case.list, test_case.partition_value);
            test_case.list.partition_stable_unsafe(test_case.partition_value);
            let mut ge_found = false;
            for item in &test_case.list
            {
                let is_ge = *item >= test_case.partition_value;
                ge_found |= is_ge;
                let symbol = if is_ge { ">=" } else { "<" };
                println!("{} {} {}", item, symbol, test_case.partition_value);
                assert!(is_ge == ge_found,
                        "item less than the partition value found after values greater than or equal to the partition value");
            }
            println!("Ok");
            println!("Output: {:?}", test_case.list);
            println!();
        }
    }

    #[test]
    fn test_partition_unstable_unsafe()
    {
        let mut test_cases = read_test_cases();
        for ref mut test_case in &mut test_cases
        {
            println!("Test Case:");
            println!("Input: {:?} {{partition_value: {}}}", test_case.list, test_case.partition_value);
            test_case.list.partition_unstable_unsafe(test_case.partition_value);
            let mut ge_found = false;
            for item in &test_case.list
            {
                let is_ge = *item >= test_case.partition_value;
                ge_found |= is_ge;
                let symbol = if is_ge { ">=" } else { "<" };
                println!("{} {} {}", item, symbol, test_case.partition_value);
                assert!(is_ge == ge_found,
                        "item less than the partition value found after values greater than or equal to the partition value");
            }
            println!("Ok");
            println!("Output: {:?}", test_case.list);
            println!();
        }
    }
}
