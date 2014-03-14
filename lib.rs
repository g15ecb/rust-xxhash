#[crate_id="rust-xxhash#0.0"];
#[crate_type="lib"];

#[deny(warnings)];
#[allow(non_camel_case_types, uppercase_variables)];

#[feature(default_type_params)];

#[cfg(test)]
extern crate test;

pub use xxhash::{XXState,XXHasher,xxh32};

#[cfg(target_endian = "big")]
#[static_assert]
static little_endian_only_sorry :bool=false;

pub mod xxhash;
