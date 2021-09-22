#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod adts;
pub mod brands;
pub mod collect;
pub mod dispatch;
pub mod errors;
pub mod macros;
pub mod memory;
pub mod orphan;
pub mod rc;
pub mod rsqrt;
pub mod typing;
