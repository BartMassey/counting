use criterion::{black_box, criterion_group, criterion_main, Criterion};
use counting::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut buffer = [0; 16];
    c.bench_function(
        "counting",
        |b| b.iter(|| {
            let mut input = RangePl::new(0..1000);
            let mut start = 0;
            loop {
                let s = input.next_batch(start, black_box(&mut buffer));
                if s == 0 {
                    return;
                }
                start = s;
            }
        }),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
