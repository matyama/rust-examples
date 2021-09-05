//! This example demonstrates differences between a *static dispatch* and
//! *dynamic dispatch* of method calls.

use std::boxed::Box;

/// Interface of a real 1D differentiable function
pub trait Differentiable {
    /// Compute the first derivative of this function at given point `x`
    fn grad(&self, x: f64) -> f64;
}

#[allow(dead_code)]
pub struct Quadratic {
    a: f64,
    b: f64,
    c: f64,
}

impl Quadratic {
    #[inline(always)]
    pub fn stack_alloc(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    #[inline(always)]
    pub fn heap_alloc(a: f64, b: f64, c: f64) -> Box<Self> {
        Box::new(Self { a, b, c })
    }
}

impl Differentiable for Quadratic {
    #[inline(always)]
    fn grad(&self, x: f64) -> f64 {
        2. * self.a * x - self.b
    }
}

#[allow(dead_code)]
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

/// Gradient Descent that finds a minimum of a statically defined function `f` on given `interval`.
///
/// Static dispatch means that this function i *monomorphized* and thus the type of `f` is known at
/// compilation time. Monomorphization means that the compiler duplicates this function for every
/// type implementing `Differentiable` and for each copy substitutes the concrete type for the
/// generic parameter `F`.
///
/// Call `f.grad(x)` is direct and the method address is explicitly mentioned in the assembly code.
/// Furthermore, trait implementations can benefit from inlining.
///
/// There's, however, also a disadvantage - one can't put different implementations into let's say a
/// collection (e.g. `Vec`) that expects homogeneous types. Such a structure need a different kind \
/// of polymorphism which is provided by dynamic dispatch.
pub fn gradient_descent_static<F>(f: &F, max_iters: usize, eta: f64) -> f64
where
    F: Differentiable,
{
    let mut x = 0.0;
    for _ in 0..max_iters {
        // Note that `F::grad(f, x)` works as well due to static dispatch and monomorphization.
        x -= eta * f.grad(x);
    }
    x
}

/// Gradient Descent that finds a minimum of a dynamically defined function `f` on given `interval`.
///
/// Dynamic dispatch means that the actual type of `f` (or more precisely of the `grad` method) is
/// determined at runtime. Call `f.grad(x)` is indirect via a *vtable* - a lookup table that holds
/// memory addresses of `grad` methods for each type implementing `Differentiable`.
///
/// Contrary to the statically dispatched version, this implementation `f` only provides the public
/// interface defined by `Differentiable` trait. Moreover any inlining on `dyn` traits is ignored.
pub fn gradient_descent_dynamic(f: &dyn Differentiable, max_iters: usize, eta: f64) -> f64 {
    let mut x = 0.0;
    for _ in 0..max_iters {
        x -= eta * f.grad(x);
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
        // min { 2*x^2 - x } = -1/8 at x = 1/4
        let function = Quadratic {
            a: 2.,
            b: 1.,
            c: 0.,
        };

        // Minimize for 10k iterations with step size 0.01

        // Test GD with static dispatch
        let x_min = gradient_descent_static(&function, 10_000, 0.01);
        assert_delta!(0.25, x_min, EPS);

        // Test GD with dynamic dispatch
        let function = Box::new(function);
        let x_min = gradient_descent_dynamic(function.as_ref(), 10_000, 0.01);
        assert_delta!(0.25, x_min, EPS);
    }

    #[test]
    fn trigonometric() {
        let function = Trigonometric::Sine;

        // Minimize for 10k iterations with step size 0.01

        // Test GD with static dispatch
        let x_min = gradient_descent_static(&function, 10_000, 0.01);
        assert_delta!(FRAC_PI_2, x_min, EPS);

        // Test GD with dynamic dispatch
        let function = Box::new(function);
        let x_min = gradient_descent_dynamic(function.as_ref(), 10_000, 0.01);
        assert_delta!(FRAC_PI_2, x_min, EPS);
    }

    #[test]
    fn dynamic_polymorphism() {
        // Define a collection of `Differentiable` functions that are heap-allocated
        //  - See [CannotMonomorphizeDifferentiableInVecTest]
        let functions: Vec<Box<dyn Differentiable>> = vec![
            Box::new(Trigonometric::Sine),
            Box::new(Trigonometric::Cosine),
        ];

        // Test that GD with dynamic dispatch works
        for function in functions.into_iter() {
            gradient_descent_dynamic(function.as_ref(), 10_000, 0.01);
        }
    }
}

/// This test shows that if one wants to construct a container ([Vec] in this case) of
/// [Differentiable] instances, it cannot be done with a *static polymorphic type*.
///
/// # Example
/// ```compile_fail
/// use rust_examples::dispatch::{Differentiable, Trigonometric};
///
/// let _: Vec<Differentiable> = vec![Trigonometric::Sine, Trigonometric::Cosine];
/// ```
///
/// The *size* of this heterogeneous container cannot be known at compile time - this issue is that
/// the compiler tries to monomorphize [Differentiable] items in the [Vec] but each one might be
/// different and the collection is dynamic, hence the unknown size.
///
/// In consequence, here one **must** use heap-allocated `dyn` instances (i.e. dynamic dispatch)!
pub struct CannotMonomorphizeDifferentiableInVecTest;
