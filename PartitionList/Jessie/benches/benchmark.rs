use std::hint::black_box;
use std::time::Duration;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use PartitionList::test::read_test_cases;

fn criterion_benchmark(c: &mut Criterion) {
    let mut test_cases = read_test_cases();
    let mut group = c.benchmark_group("partition list");
    group.bench_function("Jessie", |b| {
        b.iter(|| {
            for ref mut test_case in &mut test_cases {
                test_case
                    .list
                    .partition(black_box(test_case.partition_value));
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
