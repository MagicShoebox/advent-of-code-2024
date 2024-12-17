use std::error;

pub type SolveError = Box<dyn error::Error>;
pub type SolveResult = Result<(String, String), SolveError>;

// TODO: Generate via procedural macro
pub mod days {
    mod day01;
    pub use day01::solve as day01;
}
