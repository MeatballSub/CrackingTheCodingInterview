use std::hint::black_box;
use std::time::Duration;

use KthToLast::kth_to_last;
use KthToLast::kth_to_last_no_size;
use KthToLast::test::read_test_cases;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

fn criterion_benchmark(c: &mut Criterion)
{
    let mut test_cases = read_test_cases();
    let mut group = c.benchmark_group("k-th to last");
    group.bench_function("Kevin", |b| {
             b.iter(|| {
                  for test_case in &test_cases
                  {
                      let actual = kth_to_last(black_box(test_case.k), black_box(&test_case.list));
                      assert_eq!(actual.cloned(), test_case.answer, "Test case: {:?}\n    your answer: {:?}", test_case, actual);
                  }
                  for test_case in &test_cases
                  {
                      let actual = kth_to_last_no_size(black_box(test_case.k), black_box(&test_case.list));
                      assert_eq!(actual.cloned(), test_case.answer, "Test case: {:?}\n    your answer: {:?}", test_case, actual);
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
