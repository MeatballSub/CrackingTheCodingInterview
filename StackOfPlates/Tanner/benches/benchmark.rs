use std::hint::black_box;
use std::time::Duration;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use stack_of_plates::test::read_bench_cases;
use stack_of_plates::test::run_operations;
use stack_of_plates::SetOfStacks;

fn criterion_benchmark(c: &mut Criterion) {
    let bench_cases = read_bench_cases();
    let mut group = c.benchmark_group("stack of plates");
    group.bench_function("Tanner", |b| {
        b.iter(|| {
            for case in &bench_cases {
                let mut stack = SetOfStacks::new(case.capacity);
                run_operations(&mut stack, black_box(case));
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
