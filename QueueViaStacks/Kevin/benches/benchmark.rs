use std::hint::black_box;
use std::time::Duration;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use queue_via_stacks::NamedQueue;
use queue_via_stacks::eager_two_stack_queue::EagerTwoStackQueue;
use queue_via_stacks::lazy_two_stack_queue::LazyTwoStackQueue;
use queue_via_stacks::test::IMPLEMENTATIONS;
use queue_via_stacks::test::TestCase;
use queue_via_stacks::test::read_bench_cases;
use queue_via_stacks::test::run_operations;

const AUTHOR: &str = "Kevin";

macro_rules! bench_queue_via_stacks_impls {
    ($group:expr, $cases:expr, $($queue:ty),+ $(,)?) =>
    {
        const BENCHED: &[&str] = &[$(<$queue as NamedQueue>::NAME),+];
        const _: () = assert!(BENCHED.len() == IMPLEMENTATIONS.len(), "every entry in IMPLEMENTATIONS needs one benchmark arm, and vice versa");

        $(
            {
                const LABEL: &str = <$queue as NamedQueue>::NAME;
                IMPLEMENTATIONS.iter()
                               .find(|(name, _)| *name == LABEL)
                               .unwrap_or_else(|| panic!("{LABEL} is benchmarked but not registered in IMPLEMENTATIONS"));
                $group.bench_function(format!("{AUTHOR} - {LABEL}"), |b| {
                         b.iter(|| {
                              for case in $cases
                              {
                                  let mut queue = <$queue>::default();
                                  run_operations(&mut queue, black_box(case), LABEL);
                              }
                          })
                     });
            }
        )+
    };
}

fn criterion_benchmark(c: &mut Criterion)
{
    let bench_cases: Vec<TestCase> = read_bench_cases();
    let mut group = c.benchmark_group("queue via stacks");
    bench_queue_via_stacks_impls!(group, &bench_cases, EagerTwoStackQueue, LazyTwoStackQueue);
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(5000).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6));
    targets = criterion_benchmark
}
criterion_main!(benches);
