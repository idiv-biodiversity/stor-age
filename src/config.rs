use crate::output::Output;

#[derive(Clone, Copy)]
pub struct Config {
    pub debug: bool,
    pub verbose: bool,
    pub age_days: u64,
    pub spectrum_scale: bool,
    pub output: Output,
}
