#[crate_id="rust-hash#0.0"];
#[crate_type="lib"];

#[cfg(test)]
extern mod extra;

pub use xxhash::{XXHState,xxh32};

#[cfg(target_endian = "big")]
#[static_assert]
static little_endian_only_sorry :bool=false;

pub mod xxhash;
