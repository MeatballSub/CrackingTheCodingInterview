use std::hint::black_box;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use is_unique::is_unique;
use is_unique::test::read_test_cases;

fn criterion_benchmark(c: &mut Criterion)
{
    let test_cases = read_test_cases();
    c.bench_function("is_unique", |b| {
         b.iter(|| {
              for ref test_case in test_cases.clone()
              {
                  is_unique(black_box(&test_case.input));
              }
          })
     });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
