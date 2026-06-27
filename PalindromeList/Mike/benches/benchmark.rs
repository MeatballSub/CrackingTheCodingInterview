use std::hint::black_box;
use std::time::Duration;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use palindrome_list::is_palindrome;
use palindrome_list::test::read_test_cases;

fn criterion_benchmark(c: &mut Criterion) {
    let test_cases = read_test_cases();
    let mut group = c.benchmark_group("palindrome list");
    group.bench_function("Mike", |b| {
        b.iter(|| {
            for test_case in &test_cases {
                black_box(is_palindrome(black_box(&test_case.list)));
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
