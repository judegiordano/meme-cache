mod cache;
mod get;
mod remove;
mod set;

#[cfg(test)]
mod test;

pub use cache::{clear, entries, footprint, purge_stale, size};
pub use get::get;
pub use remove::{remove, remove_last};
pub use set::set;
