use std::hint::black_box;
use std::time::Duration;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use remove_dups::remove_dups;
use remove_dups::remove_dups_follow_up;
use remove_dups::test::read_test_cases;

fn criterion_benchmark(c: &mut Criterion) {
    let mut test_cases = read_test_cases();
    let mut group = c.benchmark_group("Remove Duplicates");
    group.bench_function("Jessie", |b| {
        b.iter(|| {
            for test_case in &mut test_cases {
                let mut actual = test_case.list.clone();
                remove_dups(black_box(&mut actual));
                assert_eq!(
                    actual, test_case.answer,
                    "Test case: {:?}\n    your answer: {:?}",
                    test_case, actual
                );
            }
            for test_case in &mut test_cases {
                let mut actual = test_case.list.clone();
                remove_dups_follow_up(black_box(&mut actual));
                assert_eq!(
                    actual, test_case.answer,
                    "Test case: {:?}\n    your answer: {:?}",
                    test_case, actual
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
