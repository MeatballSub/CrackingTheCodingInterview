use std::time::Duration;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use stack_min::test::read_test_cases;
use stack_min::test::run_operations;
use stack_min::MinStack;

fn criterion_benchmark(c: &mut Criterion) {
    let test_cases = read_test_cases();
    let mut group = c.benchmark_group("stack min");
    group.bench_function("Jessie", |b| {
        b.iter(|| {
            for case in &test_cases {
                let mut stack = MinStack::new();
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
