use std::hint::black_box;
use std::time::Duration;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use urlify::test::read_test_cases;
use urlify::urlify_kevin;
use urlify::urlify_mike;
use urlify::urlify_tanner;

fn criterion_benchmark(c: &mut Criterion)
{
    let test_cases = read_test_cases();
    let mut group = c.benchmark_group("URLify");
    group.bench_function("kevin", |b| {
             b.iter(|| {
                  for test_case in &test_cases
                  {
                      let mut input_string = test_case.input_string.clone();
                      urlify_kevin(black_box(&mut input_string), black_box(test_case.input_true_length));
                  }
              })
         });
    group.bench_function("tanner", |b| {
             b.iter(|| {
                  for test_case in &test_cases
                  {
                      let mut input_string = test_case.input_string.clone();
                      urlify_tanner(black_box(&mut input_string), black_box(test_case.input_true_length));
                  }
              })
         });
    group.bench_function("mike", |b| {
             b.iter(|| {
                  for test_case in &test_cases
                  {
                      let mut input_string = test_case.input_string.clone();
                      urlify_mike(black_box(&mut input_string), black_box(test_case.input_true_length));
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
