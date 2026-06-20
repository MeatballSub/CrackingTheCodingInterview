use std::hint::black_box;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BatchSize;
use criterion::Criterion;
use sum_lists::linked_list::LinkedList;

const MAX_STACK: usize = 1000;
const MAX_DIGITS: usize = MAX_STACK - 1;

fn make_lists() -> Vec<(LinkedList<u8>, LinkedList<u8>)> {
    let range = 0..MAX_DIGITS;
    let big_a: LinkedList<u8> = range.clone().map(|i| (i % 9) as u8).collect();
    let big_b: LinkedList<u8> = range.map(|i| (8 - (i % 9)) as u8).collect();

    vec![
        // equal length
        (
            vec![6, 1, 7].into_iter().collect(),
            vec![2, 9, 5].into_iter().collect(),
        ),
        // unequal length
        (
            vec![9, 9].into_iter().collect(),
            vec![1].into_iter().collect(),
        ),
        // large carry chain
        (
            vec![9, 9, 9].into_iter().collect(),
            vec![9, 9, 9].into_iter().collect(),
        ),
        // single digit
        (vec![5].into_iter().collect(), vec![5].into_iter().collect()),
        // long lists
        (
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect(),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1].into_iter().collect(),
        ),
        // large amount of digits
        (big_a, big_b),
    ]
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum lists");

    group.bench_function("Tanner - sum_lists_reverse", |b| {
        b.iter_batched(
            make_lists,
            |lists| {
                for (a, b) in lists {
                    sum_lists::sum_lists_reverse(black_box(a), black_box(b));
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Tanner - sum_lists_forward", |b| {
        b.iter_batched(
            make_lists,
            |lists| {
                for (a, b) in lists {
                    sum_lists::sum_lists_forward(black_box(a), black_box(b));
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
}
criterion_main!(benches);
