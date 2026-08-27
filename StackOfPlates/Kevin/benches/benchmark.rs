use std::hint::black_box;
use std::time::Duration;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use stack_of_plates::flat_stacks::FlatStacks;
use stack_of_plates::no_rollover_stacks::NoRolloverStacks;
use stack_of_plates::rollover_stacks::RolloverStacks;
use stack_of_plates::test::IMPLEMENTATIONS;
use stack_of_plates::test::TestCase;
use stack_of_plates::test::read_bench_cases;
use stack_of_plates::test::run_operations;

const AUTHOR: &str = "Kevin";

macro_rules! bench_stack_of_plates_impls {
    ($group:expr, $cases:expr, $($label:literal => $ctor:expr),+ $(,)?) =>
    {
        const BENCHED: &[&str] = &[$($label),+];
        const _: () = assert!(BENCHED.len() == IMPLEMENTATIONS.len(), "every entry in IMPLEMENTATIONS needs one benchmark arm, and vice versa");

        $(
            {
                IMPLEMENTATIONS.iter()
                               .find(|(name, _, _)| *name == $label)
                               .expect(concat!("no IMPLEMENTATIONS entry named ", $label));
                $group.bench_function(format!("{AUTHOR} - {}", $label), |b| {
                         b.iter(|| {
                              for case in $cases
                              {
                                  let mut stack = $ctor(case.capacity);
                                  run_operations(&mut stack, black_box(case), $label);
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
    let mut group = c.benchmark_group("stack of plates");
    bench_stack_of_plates_impls!(group, &bench_cases, "no_rollover_stacks" => NoRolloverStacks::new, "rollover_stacks" => RolloverStacks::new,
                                                     "flat_stacks" => FlatStacks::new);
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(5000).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6));
    targets = criterion_benchmark
}
criterion_main!(benches);
