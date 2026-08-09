use std::hint::black_box;
use std::time::Duration;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use stack_min::auxiliary_min_stack::AuxiliaryMinStack;
use stack_min::paired_min_stack::PairedMinStack;
use stack_min::test::IMPLEMENTATIONS;
use stack_min::test::TestCase;
use stack_min::test::read_test_cases;
use stack_min::test::run_operations;

const AUTHOR: &str = "Kevin";

macro_rules! bench_stack_min_impls {
    ($group:expr, $cases:expr, $($label:literal => $ctor:expr),+ $(,)?) =>
    {
        // Guards against the two registries drifting apart: an implementation
        // added to IMPLEMENTATIONS but not benchmarked (or the reverse) fails
        // the build instead of silently going unmeasured.
        const BENCHED: &[&str] = &[$($label),+];
        const _: () = assert!(BENCHED.len() == IMPLEMENTATIONS.len(), "every entry in IMPLEMENTATIONS needs one benchmark arm, and vice versa");

        $(
            $group.bench_function(format!("{AUTHOR} - {}", $label), |b| {
                     b.iter(|| {
                          for case in $cases
                          {
                              let mut stack = $ctor();
                              run_operations(&mut stack, black_box(case), $label);
                          }
                      })
                 });
        )+
    };
}

fn criterion_benchmark(c: &mut Criterion)
{
    let test_cases: Vec<TestCase> = read_test_cases();
    let mut group = c.benchmark_group("stack min");
    bench_stack_min_impls!(group, &test_cases, "paired_min_stack" => PairedMinStack::new, "auxiliary_min_stack" => AuxiliaryMinStack::new);
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(5000).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6));
    targets = criterion_benchmark
}
criterion_main!(benches);
