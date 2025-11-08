use std::hint::black_box;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use urlify::test::read_test_cases;
use urlify::urlify;

fn criterion_benchmark(c: &mut Criterion) {
    let test_cases = read_test_cases();
    c.bench_function("urlify", |b| {
        b.iter(|| {
            for test_case in &test_cases {
                let mut input_string = test_case.input_string.clone();
                urlify(
                    black_box(&mut input_string),
                    black_box(test_case.input_true_length),
                );
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
