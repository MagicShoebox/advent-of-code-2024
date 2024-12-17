use advent_of_code_2024::days::*;
use advent_of_code_2024::*;
use std::{env, fmt, fs, io, num::ParseIntError, process};

// TODO: Replace with <u32 as FromStr>::Err when issue
// https://github.com/rust-lang/rust/issues/85576
// is fixed.
type ParseDayError = ParseIntError;

const DAYS: [fn(&str) -> SolveResult; 1] = [day01];

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(2);
    });
    if let Err(err) = run(config) {
        println!("{err}");
        process::exit(1);
    }
}

struct Config<'a> {
    day: usize,
    filename: &'a str,
}

enum ConfigError<'a> {
    WrongNumberOfParameters {
        program_name: &'a str,
        expected: usize,
        actual: usize,
    },
    InvalidDay(DayError),
}

enum DayError {
    OutOfRange {
        low: usize,
        high: usize,
        actual: usize,
    },
    ParseError(ParseDayError),
}

enum RunError<'a> {
    FileError {
        filename: &'a str,
        error: io::Error
    },
    SolveError(SolveError)
}

impl fmt::Display for ConfigError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ConfigError::WrongNumberOfParameters { program_name, expected, actual }
                => write!(f, "Expected {expected} parameters. Found {actual}.\nUsage: {program_name} day filename"),
            ConfigError::InvalidDay(day_error)
                => day_error.fmt(f),
        }
    }
}

impl fmt::Display for DayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &DayError::OutOfRange { low, high, actual } => {
                write!(
                    f,
                    "Day must be between {low} and {high} (inclusive). Found {actual}."
                )
            }
            DayError::ParseError(parse_int_error) => {
                write!(f, "Invalid day: ")?;
                parse_int_error.fmt(f)
            }
        }
    }
}

impl fmt::Display for RunError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::FileError { filename, error } => {
                write!(f, "Error reading {filename}: ")?;
                error.fmt(f)
            },
            RunError::SolveError(err) => {
                err.fmt(f)
            }
        }
    }
}

impl From<DayError> for ConfigError<'_> {
    fn from(value: DayError) -> Self {
        ConfigError::InvalidDay(value)
    }
}

impl From<ParseDayError> for DayError {
    fn from(value: ParseDayError) -> Self {
        DayError::ParseError(value)
    }
}

impl From<SolveError> for RunError<'_> {
    fn from(value: SolveError) -> Self {
        RunError::SolveError(value)
    }
}

impl Config<'_> {
    fn build(args: &[String]) -> Result<Config, ConfigError> {
        const EXPECTED: usize = 2;
        let program_name = if args.len() > 0 { &args[0] } else { "solve" };
        if args.len() != EXPECTED + 1 {
            return Err(ConfigError::WrongNumberOfParameters {
                program_name,
                expected: EXPECTED,
                actual: args.len() - 1,
            });
        }

        Ok(Config {
            day: Self::parse_day(&args[1])?,
            filename: &args[2],
        })
    }

    fn parse_day(day: &str) -> Result<usize, DayError> {
        let day: usize = day.parse()?;
        if day == 0 || day > DAYS.len() {
            return Err(DayError::OutOfRange {
                low: 1,
                high: DAYS.len(),
                actual: day,
            });
        }

        Ok(day)
    }
}

fn run(config: Config) -> Result<(), RunError> {
    let input = fs::read_to_string(config.filename)
        .map_err(|error| RunError::FileError { filename: config.filename, error })?;
    println!("Solving day {} with {}", config.day, config.filename);
    let (part1, part2) = DAYS[config.day - 1](&input)?;
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
