use std::hint::black_box;
use std::time::Duration;

use animal_shelter::test::read_bench_cases;
use animal_shelter::test::run_operations;
use animal_shelter::AnimalShelter;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    let bench_cases = read_bench_cases();
    let mut group = c.benchmark_group("animal shelter");
    group.bench_function("Tanner", |b| {
        b.iter(|| {
            for case in &bench_cases {
                let mut shelter = AnimalShelter::new();
                run_operations(&mut shelter, black_box(case));
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
