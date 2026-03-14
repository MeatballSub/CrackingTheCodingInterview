use std::hint::black_box;

use check_permutation::check_permutation;
use check_permutation::test::read_test_cases;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    let test_cases = read_test_cases();
    c.bench_function("check_permutation", |b| {
        b.iter(|| {
            for ref test_case in test_cases.clone() {
                check_permutation(black_box(&test_case.input1), black_box(&test_case.input2));
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
