extern crate rust_examples;
use rust_examples::dispatch::*;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::SeedableRng;
use rand_pcg::Pcg64;

fn bench_quadratic(c: &mut Criterion) {
    // Define a benchmark group for simple Gradient Descent on a quadratic function
    let mut group = c.benchmark_group("GD - Quadratic");

    // Benchmark GD with static dispatch
    group.bench_function("Static Dispatch", |b| {
        // Since we always run GD for fixed number of iterations, it shouldn't matter
        // that we re-use this RNG in all the benches
        let mut rng = Pcg64::seed_from_u64(42);

        let function = Quadratic::alloc_stack(2., 1., 0.);

        b.iter(|| gradient_descent_static(&function, -0.5..=0.5, 10_000, 0.01, &mut rng));
    });

    // Benchmark GD with dynamic dispatch
    group.bench_function("Dynamic Dispatch", |b| {
        let mut rng = Pcg64::seed_from_u64(42);

        let function = Quadratic::alloc_heap(2., 1., 0.);

        b.iter(|| gradient_descent_dynamic(function.as_ref(), -0.5..=0.5, 10_000, 0.01, &mut rng));
    });

    group.finish();
}

criterion_group!(benches, bench_quadratic);
criterion_main!(benches);
