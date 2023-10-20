#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod analysis;
mod data;
pub mod output;

#[cfg(feature = "spectrum-scale")]
pub use analysis::spectrum_scale::run as spectrum_scale;
pub use analysis::universal::run as universal;
pub use data::Data;
