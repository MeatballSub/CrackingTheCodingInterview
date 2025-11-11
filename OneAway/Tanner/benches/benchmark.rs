use std::hint::black_box;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use one_away::one_away;
use one_away::test::read_test_cases;

fn criterion_benchmark(c: &mut Criterion)
{
    let test_cases = read_test_cases();
    c.bench_function("one_away", |b| {
         b.iter(|| {
              for test_case in &test_cases
              {
                  one_away(black_box(&test_case.string1), black_box(&test_case.string2));
              }
          })
     });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
