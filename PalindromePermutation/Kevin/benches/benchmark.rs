use std::hint::black_box;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use palindrome_permutation::palindrome_permutation;
use palindrome_permutation::test::read_test_cases;

fn criterion_benchmark(c: &mut Criterion)
{
    let test_cases = read_test_cases();
    c.bench_function("palindrome_permutation", |b| {
         b.iter(|| {
              for test_case in &test_cases
              {
                  palindrome_permutation(black_box(&test_case.input));
              }
          })
     });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
