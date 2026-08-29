use std::hint::black_box;
use std::time::Duration;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use queue_via_stacks::test::read_bench_cases;
use queue_via_stacks::test::run_operations;
use queue_via_stacks::MyQueue;

fn criterion_benchmark(c: &mut Criterion) {
    let bench_cases = read_bench_cases();
    let mut group = c.benchmark_group("queue via stacks");
    group.bench_function("Mike", |b| {
        b.iter(|| {
            for case in &bench_cases {
                let mut queue = MyQueue::new();
                run_operations(&mut queue, black_box(case));
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
