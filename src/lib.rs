mod cache;
mod get;
mod set;

#[cfg(test)]
mod test;

pub use cache::{clear, footprint, size};
pub use get::get;
pub use set::set;
