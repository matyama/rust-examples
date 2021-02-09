//! This example demonstrates differences between a *static dispatch* and
//! *dynamic dispatch* of method calls.

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

use std::boxed::Box;
use std::marker::Sized;
use std::ops::RangeInclusive;

pub trait Differentiable {
    fn grad(&self, x: f64) -> f64;
}

pub struct Quadratic {
    a: f64,
    b: f64,
    c: f64,
}

impl Quadratic {
    #[inline(always)]
    pub fn alloc_stack(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    #[inline(always)]
    pub fn alloc_heap(a: f64, b: f64, c: f64) -> Box<Self> {
        Box::new(Self { a, b, c })
    }
}

impl Differentiable for Quadratic {
    #[inline(always)]
    fn grad(&self, x: f64) -> f64 {
        2. * self.a * x - self.b
    }
}

enum Trigonometric {
    Sine,
    Cosine,
}

impl Differentiable for Trigonometric {
    #[inline(always)]
    fn grad(&self, x: f64) -> f64 {
        match self {
            Trigonometric::Sine => x.cos(),
            Trigonometric::Cosine => -x.sin(),
        }
    }
}

pub fn gradient_descent_static<F, R>(
    f: &F,
    interval: RangeInclusive<f64>,
    max_iters: usize,
    eta: f64,
    rng: &mut R,
) -> f64
where
    F: Differentiable,
    R: Rng + ?Sized,
{
    let mut x = rng.gen_range(interval);
    for i in 0..max_iters {
        x = x - eta * f.grad(x);
    }
    x
}

pub fn gradient_descent_dynamic<R>(
    f: &dyn Differentiable,
    interval: RangeInclusive<f64>,
    max_iters: usize,
    eta: f64,
    rng: &mut R,
) -> f64
where
    R: Rng + ?Sized,
{
    let mut x = rng.gen_range(interval);
    for i in 0..max_iters {
        x = x - eta * f.grad(x);
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    const EPS: f64 = 0.00001;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!("{} is not within {} of {}", $x, $d, $y);
            }
        };
    }

    #[test]
    fn quadratic() {
        let mut rng = Pcg64::seed_from_u64(42);

        // min { 2*x^2 - x } = -1/8 at x = 1/4
        let function = Quadratic {
            a: 2.,
            b: 1.,
            c: 0.,
        };

        // Minimize on interval [-1/2, 1/2] for 10k iterations with step size 0.01

        // Test GD with static dispatch
        let x_min = gradient_descent_static(&function, -0.5..=0.5, 10_000, 0.01, &mut rng);
        assert_delta!(0.25, x_min, EPS);

        // Test GD with dynamic dispatch
        let function = Box::new(function);
        let x_min = gradient_descent_dynamic(function.as_ref(), -0.5..=0.5, 10_000, 0.01, &mut rng);
        assert_delta!(0.25, x_min, EPS);
    }

    #[test]
    fn trigonometric() {
        let mut rng = Pcg64::seed_from_u64(42);

        let function = Trigonometric::Sine;

        // Minimize on interval [-5, 3.5] for 10k iterations with step size 0.01

        // Test GD with static dispatch
        let x_min = gradient_descent_static(&function, -5.0..=3.5, 10_000, 0.01, &mut rng);
        assert_delta!(FRAC_PI_2, x_min, EPS);

        // Test GD with dynamic dispatch
        let function = Box::new(function);
        let x_min = gradient_descent_dynamic(function.as_ref(), -5.0..=3.5, 10_000, 0.01, &mut rng);
        assert_delta!(FRAC_PI_2, x_min, EPS);
    }
}
