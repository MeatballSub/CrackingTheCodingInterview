use std::hint::black_box;
use std::time::Duration;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use palindrome_permutation::palindrome_permutation_kevin_1;
use palindrome_permutation::palindrome_permutation_kevin_2;
use palindrome_permutation::palindrome_permutation_kevin_3;
use palindrome_permutation::palindrome_permutation_mike;
use palindrome_permutation::test::read_test_cases;

fn criterion_benchmark(c: &mut Criterion)
{
    let test_cases = read_test_cases();
    c.bench_function("palindrome_permutation_kevin_1", |b| {
         b.iter(|| {
              for test_case in &test_cases
              {
                  palindrome_permutation_kevin_1(black_box(&test_case.input));
              }
          })
     });
    c.bench_function("palindrome_permutation_kevin_2", |b| {
         b.iter(|| {
              for test_case in &test_cases
              {
                  palindrome_permutation_kevin_2(black_box(&test_case.input));
              }
          })
     });
    c.bench_function("palindrome_permutation_kevin_3", |b| {
         b.iter(|| {
              for test_case in &test_cases
              {
                  palindrome_permutation_kevin_3(black_box(&test_case.input));
              }
          })
     });
    c.bench_function("palindrome_permutation_mike", |b| {
         b.iter(|| {
              for test_case in &test_cases
              {
                  palindrome_permutation_mike(black_box(&test_case.input));
              }
          })
     });
}

// criterion_group!(benches, criterion_benchmark);
criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(5000).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6));
    targets = criterion_benchmark
}
criterion_main!(benches);
