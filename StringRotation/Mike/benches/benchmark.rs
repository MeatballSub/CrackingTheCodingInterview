use std::hint::black_box;
use std::time::Duration;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use string_rotation::string_rotation;
use string_rotation::test::read_test_cases;
use string_rotation::LimitedSubstring;

fn criterion_benchmark(c: &mut Criterion) {
    let mut test_cases = read_test_cases();
    let mut group = c.benchmark_group("String Rotation");
    group.bench_function("Mike", |b| {
        b.iter(|| {
            for test_case in &mut test_cases {
                let mut is_substring_provider = LimitedSubstring::new();
                let closure = |s1: &str, s2: &str| is_substring_provider.is_substring(s1, s2);
                string_rotation(
                    black_box(&test_case.s1),
                    black_box(&test_case.s2),
                    black_box(closure),
                );
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
