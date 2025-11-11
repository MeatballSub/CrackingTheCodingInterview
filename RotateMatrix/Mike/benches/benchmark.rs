use std::hint::black_box;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use rotate_matrix::rotate_matrix;
use rotate_matrix::test::read_test_cases;

fn criterion_benchmark(c: &mut Criterion) {
    let test_cases = read_test_cases();
    c.bench_function("rotate_matrix", |b| {
        b.iter(|| {
            for test_case in &test_cases {
                rotate_matrix(black_box(&test_case.matrix));
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
