use std::hint::black_box;
use std::time::Duration;

use animal_shelter::NamedShelter;
use animal_shelter::test::IMPLEMENTATIONS;
use animal_shelter::test::TestCase;
use animal_shelter::test::read_bench_cases;
use animal_shelter::test::run_operations;
use animal_shelter::two_queue_shelter::TwoQueueShelter;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

const AUTHOR: &str = "Kevin";

macro_rules! bench_animal_shelter_impls {
    ($group:expr, $cases:expr, $($shelter:ty),+ $(,)?) =>
    {
        const BENCHED: &[&str] = &[$(<$shelter as NamedShelter>::NAME),+];
        const _: () = assert!(BENCHED.len() == IMPLEMENTATIONS.len(), "every entry in IMPLEMENTATIONS needs one benchmark arm, and vice versa");

        $(
            {
                const LABEL: &str = <$shelter as NamedShelter>::NAME;
                IMPLEMENTATIONS.iter()
                               .find(|(name, _)| *name == LABEL)
                               .unwrap_or_else(|| panic!("{LABEL} is benchmarked but not registered in IMPLEMENTATIONS"));
                $group.bench_function(format!("{AUTHOR} - {LABEL}"), |b| {
                         b.iter(|| {
                              for case in $cases
                              {
                                  let mut shelter = <$shelter>::default();
                                  run_operations(&mut shelter, black_box(case), LABEL);
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
    let mut group = c.benchmark_group("animal shelter");
    bench_animal_shelter_impls!(group, &bench_cases, TwoQueueShelter);
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(5000).measurement_time(Duration::from_secs(10)).warm_up_time(Duration::from_secs(6));
    targets = criterion_benchmark
}
criterion_main!(benches);
