use std::hint::black_box;
use std::time::Duration;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use three_in_one::FixedMultiStack;
use three_in_one::ThreeStacks;
use three_in_one::test::read_test_cases;
use three_in_one::test::run_operations;

const AUTHOR: &str = "Kevin";

macro_rules! bench_three_in_one_impls {
    ($group:expr, $cases:expr, $($label:literal => $ctor:expr),+ $(,)?) =>
    {
        $(
            $group.bench_function(format!("{AUTHOR} - {}", $label), |b| {
                     b.iter(|| {
                          for case in $cases
                          {
                              let mut stack: Box<dyn ThreeStacks> = Box::new($ctor(black_box(case.stack_capacity)));
                              run_operations(&mut *stack, case, $label);
                          }
                      })
                 });
        )+
    };
}

fn criterion_benchmark(c: &mut Criterion)
{
    let test_cases = read_test_cases();
    let mut group = c.benchmark_group("three in one");
    bench_three_in_one_impls!(group, &test_cases, "fixed_multi_stack" => FixedMultiStack::new);
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(5000).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6));
    targets = criterion_benchmark
}
criterion_main!(benches);
