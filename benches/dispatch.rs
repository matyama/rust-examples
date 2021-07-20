extern crate rust_examples;
use rust_examples::dispatch::*;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_quadratic(c: &mut Criterion) {
    // Define a benchmark group for simple Gradient Descent on a quadratic function
    let mut group = c.benchmark_group("GD - Quadratic");

    // Benchmark GD with static dispatch
    group.bench_function("Static Dispatch", |b| {
        let function = Quadratic::stack_alloc(2., 1., 0.);
        b.iter(|| gradient_descent_static(&function, 10_000, 0.01));
    });

    // Benchmark GD with dynamic dispatch
    group.bench_function("Dynamic Dispatch", |b| {
        let function = Quadratic::heap_alloc(2., 1., 0.);
        b.iter(|| gradient_descent_dynamic(function.as_ref(), 10_000, 0.01));
    });

    group.finish();
}

criterion_group!(benches, bench_quadratic);
criterion_main!(benches);
