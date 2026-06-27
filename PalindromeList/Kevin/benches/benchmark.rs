use std::hint::black_box;
use std::time::Duration;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use palindrome_list::test::read_test_cases;

const AUTHOR: &str = "Kevin";

macro_rules! bench_palindrome_impls {
    ($group:expr, $cases:expr, $($func:ident),+ $(,)?) =>
    {
        $(
            {
                let name = stringify!($func);
                let id = name.strip_prefix("is_palindrome_").unwrap_or(name);
                $group.bench_function(format!("{AUTHOR} - {id}"), |b| {
                         b.iter(|| {
                              for case in $cases
                              {
                                  black_box(palindrome_list::$func(black_box(&case.list)));
                              }
                          })
                     });
            }
        )+
    };
}

fn criterion_benchmark(c: &mut Criterion)
{
    let test_cases = read_test_cases();
    let mut group = c.benchmark_group("palindrome list");
    bench_palindrome_impls!(group,
                            &test_cases,
                            is_palindrome_reverse_and_compare,
                            is_palindrome_to_vec_reverse_and_compare,
                            is_palindrome_deque,
                            is_palindrome_two_pointer,
                            is_palindrome_stack_half,
                            is_palindrome_reverse_half);
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(5000).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6));
    targets = criterion_benchmark
}
criterion_main!(benches);
