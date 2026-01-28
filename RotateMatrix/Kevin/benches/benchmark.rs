use std::hint::black_box;
use std::time::Duration;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use rotate_matrix::rotate_matrix_kevin;
use rotate_matrix::rotate_matrix_tanner;
use rotate_matrix::test::read_test_cases;

fn criterion_benchmark(c: &mut Criterion)
{
    let mut test_cases = read_test_cases();
    let mut group = c.benchmark_group("Rotate Matrix");
    group.bench_function("Kevin", |b| {
             b.iter(|| {
                  for test_case in &mut test_cases
                  {
                      rotate_matrix_kevin(black_box(&mut test_case.matrix));
                  }
              })
         });
    group.bench_function("Tanner", |b| {
             b.iter(|| {
                  for test_case in &mut test_cases
                  {
                      rotate_matrix_tanner(black_box(&mut test_case.matrix));
                  }
              })
         });
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(5000).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6));
    targets = criterion_benchmark
}
criterion_main!(benches);
