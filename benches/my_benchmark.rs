use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dice_simulation::simulate;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simulation", |b| {
        b.iter(|| simulate(black_box(10_000), black_box(5)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
