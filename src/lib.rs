#![forbid(unsafe_code)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod analysis;
mod data;
pub mod output;

#[cfg(feature = "storage-scale")]
pub use analysis::storage_scale::run as storage_scale;
pub use analysis::universal::run as universal;
pub use data::Data;
