use crate::timer::Timer;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Run an advent of code solution.
///
/// This function assumes the solving function takes the whole input as a string.
/// This assumes the entire input can be safely read into a string buffer in memory, which is usually the case.
///
/// # Errors
///
/// This function returns any I/O errors.
pub fn run<P, F, T>(name: &str, file: P, solver: F) -> Result<T, Box<dyn Error>>
where
    P: AsRef<Path>,
    F: FnOnce(&str) -> T,
    T: Display,
{
    let mut timer = Timer::new().start();
    let mut buffer = String::new();
    File::open(file)?.read_to_string(&mut buffer)?;
    println!("[{name}]: Read input in {}", timer.lap());
    let result = solver(buffer.trim());
    println!("[{name}]: Solve complete in {}", timer.lap());
    println!("[{name}]: Total time is {}", timer.stop());
    println!("[{name}]: Result is {result}");
    Ok(result)
}
