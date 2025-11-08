use std::hint::black_box;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use is_unique::is_unique_hash_set;
use is_unique::is_unique_no_data_structures;
use is_unique::is_unique_seven_bit_ascii;
use is_unique::test::read_test_cases;

fn criterion_benchmark(c: &mut Criterion)
{
    let test_cases = read_test_cases();
    c.bench_function("is_unique_hash_set", |b| {
         b.iter(|| {
              for ref test_case in test_cases.clone()
              {
                  is_unique_hash_set(black_box(&test_case.input));
              }
          })
     });

    c.bench_function("is_unique_no_data_structures", |b| {
         b.iter(|| {
              for ref test_case in test_cases.clone()
              {
                  is_unique_no_data_structures(black_box(&test_case.input));
              }
          })
     });

    c.bench_function("is_unique_seven_bit_ascii", |b| {
         b.iter(|| {
              for ref test_case in test_cases.clone()
              {
                  is_unique_seven_bit_ascii(black_box(&test_case.input));
              }
          })
     });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
