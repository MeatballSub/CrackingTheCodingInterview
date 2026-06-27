use std::time::Duration;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use three_in_one::test::read_test_cases;
use three_in_one::test::run_operations;
use three_in_one::FixedMultiStack;

fn criterion_benchmark(c: &mut Criterion) {
    let test_cases = read_test_cases();
    let mut group = c.benchmark_group("three in one");
    group.bench_function("Tanner", |b| {
        b.iter(|| {
            for case in &test_cases {
                let mut stack = FixedMultiStack::new(case.stack_capacity);
                run_operations(&mut stack, case);
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
