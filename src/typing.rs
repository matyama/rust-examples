use std::cmp::Ordering;

// Note this could be generalized and made even safer with
// `struct Positive<F: num::NotNull>(F)`
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Positive(f64);

impl Positive {
    // This forces clients to always check if it's ok
    pub fn new(number: f64) -> Option<Self> {
        if !number.is_sign_positive() {
            return None;
        }
        Some(Positive(number))
    }

    // Note: This is not OOP, I can call `Positive::as_u32(p)`
    #[inline(always)]
    unsafe fn as_u32(&self) -> u32 {
        self.0.to_int_unchecked::<u32>()
    }
}

// Positive 1: Clients don't have to check for anything on the call side.
// Positive 2: This call plays nicely with other APIs.
// Problem 1: Call to this function might crash the program! Also clients must somehow know this (e.g. read the docs).
// Problem 2: Each call adds a branching instruction which might get significant when called frequently
// Problem 3: Typical protection against Problem 1 will be adding the same check and thus duplicating the code
pub fn cmp_f64(a: f64, b: f64) -> Ordering {
    if !a.is_sign_positive() || !b.is_sign_positive() {
        panic!("Both a and b must be positive reals");
    }

    unsafe {
        let a: u32 = a.to_int_unchecked();
        let b: u32 = b.to_int_unchecked();
        a.cmp(&b)
    }
}

// Problem 1: Clients are forced to handle None even though they know it's never returned
// Problem 2: Each call adds a branching instruction which might get significant when called frequently
// Problem 3: If this is called frequently, the initial check if sign is positive will soon get costly
pub fn better_cmp_f64(a: f64, b: f64) -> Option<Ordering> {
    if !a.is_sign_positive() || !b.is_sign_positive() {
        return None;
    }

    unsafe {
        let a: u32 = a.to_int_unchecked();
        let b: u32 = b.to_int_unchecked();
        Some(a.cmp(&b))
    }
}

// Positive 1: Now his operation is completely safe even though it uses unsafe
//  - Let me stress this out: It is not possible to compile and run a program which calls this function with a negative number!
//  - This is a form of theorem proving done by the compiler and thus much stronger result than any (unit) test!
// Positive 2: The constructor of the wrapper type pushes the clients to check for errors early on
// Positive 3: The wrapper type carries certain semantics which can be taken to a benefit in the implementation
// Problem: Clients must wrap their data into the wrapper type which might get tedious and not worth it for non-critical data flows.
pub fn safe_cmp_f64(a: Positive, b: Positive) -> Ordering {
    // Rust's `unsafe` keyword is basically saying: Hey, Borrow Checker, I'm now resposible for the semantics of the memory management
    // in this block of code. Note that any crash in this scope results in an undefined behavior!
    unsafe { a.as_u32().cmp(&b.as_u32()) }
}

// TODO: http://codercorner.com/RadixSortRevisited.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = Positive::new(42.).expect("This is a positive float!");
        let b = Positive::new(24.).expect("This is a positive float!");

        // You cannot initialize a tuple struct which contains private fields.
        // So, one is forced to use the factory method and therefore check the result.
        // let n = Positive(-24.);

        assert_eq!(Ordering::Greater, safe_cmp_f64(a, b));
        assert_eq!(Ordering::Less, safe_cmp_f64(b, a));
        assert_eq!(Ordering::Equal, safe_cmp_f64(a, a));
    }
}
