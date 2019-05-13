use crate::output::Output;

#[derive(Clone, Copy)]
pub struct Config {
    pub debug: bool,
    pub age_days: u64,
    pub output: Output,

    #[cfg(feature = "spectrum-scale")]
    pub spectrum_scale: bool,
}
