#[crate_id="rust-hash#0.0"];
#[crate_type="lib"];

#[cfg(test)]
extern mod extra;

pub use xxhash::{xxh32};
pub mod xxhash;
