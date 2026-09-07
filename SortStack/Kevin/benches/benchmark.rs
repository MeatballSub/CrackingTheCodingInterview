use std::hint::black_box;
use std::time::Duration;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use sort_stack::NamedStack;
use sort_stack::insertion_sort_stack::InsertionSortStack;
use sort_stack::test::IMPLEMENTATIONS;
use sort_stack::test::TestCase;
use sort_stack::test::read_bench_cases;
use sort_stack::test::run_operations;

const AUTHOR: &str = "Kevin";

macro_rules! bench_sort_stack_impls {
    ($group:expr, $cases:expr, $($stack:ty),+ $(,)?) =>
    {
        const BENCHED: &[&str] = &[$(<$stack as NamedStack>::NAME),+];
        const _: () = assert!(BENCHED.len() == IMPLEMENTATIONS.len(), "every entry in IMPLEMENTATIONS needs one benchmark arm, and vice versa");

        $(
            {
                const LABEL: &str = <$stack as NamedStack>::NAME;
                IMPLEMENTATIONS.iter()
                               .find(|(name, _)| *name == LABEL)
                               .unwrap_or_else(|| panic!("{LABEL} is benchmarked but not registered in IMPLEMENTATIONS"));
                $group.bench_function(format!("{AUTHOR} - {LABEL}"), |b| {
                         b.iter(|| {
                              for case in $cases
                              {
                                  let mut stack = <$stack>::default();
                                  run_operations(&mut stack, black_box(case), LABEL);
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
    let mut group = c.benchmark_group("sort stack");
    bench_sort_stack_impls!(group, &bench_cases, InsertionSortStack);
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(5000).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6));
    targets = criterion_benchmark
}
criterion_main!(benches);
