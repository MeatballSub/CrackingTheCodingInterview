use std::hint::black_box;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use rotate_matrix::rotate_matrix;
use rotate_matrix::test::read_test_cases;

fn criterion_benchmark(c: &mut Criterion)
{
    let mut test_cases = read_test_cases();
    c.bench_function("rotate_matrix", |b| {
         b.iter(|| {
              for test_case in &mut test_cases
              {
                  rotate_matrix(black_box(&mut test_case.matrix));
              }
          })
     });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
