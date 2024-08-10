mod cache;
mod get;
mod set;

#[cfg(test)]
mod test;

pub use cache::{clear, entries, footprint, purge_stale, size};
pub use get::get;
pub use set::set;
