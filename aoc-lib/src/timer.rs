use std::fmt::Display;
use std::time::{Duration, Instant};

/// Time how long operations in the advent of code take.
///
/// The timer supports an overall time via start/stop, and a lap timer for measuring individual operations.
///
/// ```
/// use aoc_lib::timer::Timer;
///
/// let mut timer = Timer::new().start();
/// println!("Lap 1: {}", timer.lap());
/// println!("Lap 2: {}", timer.lap());
/// println!("Total: {}", timer.stop());
/// ```
#[derive(Debug)]
pub struct Timer<T> {
    state: T,
}

#[derive(Debug)]
pub struct Initial {}

#[derive(Debug)]
pub struct Running {
    initial: Instant,
    last_lap: Instant,
}

impl Timer<Initial> {
    #[must_use]
    pub fn new() -> Self {
        Timer { state: Initial {} }
    }

    #[must_use]
    pub fn start(self) -> Timer<Running> {
        let start = Instant::now();
        Timer {
            state: Running {
                initial: start,
                last_lap: start,
            },
        }
    }
}

impl Default for Timer<Initial> {
    fn default() -> Self {
        Timer::new()
    }
}

impl Timer<Running> {
    pub fn lap(&mut self) -> Time {
        let lap = Instant::now();
        let lap_time = lap - self.state.last_lap;
        self.state.last_lap = lap;
        lap_time.into()
    }

    #[must_use]
    pub fn stop(self) -> Time {
        Instant::elapsed(&self.state.initial).into()
    }
}

#[derive(Debug)]
pub enum Time {
    Seconds(f32),
    Millis(u128),
    Micros(u128),
    Nanos(u128),
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Time::Seconds(seconds) => write!(f, "{seconds:.3}s"),
            Time::Millis(millis) => write!(f, "{millis}ms"),
            Time::Micros(micros) => write!(f, "{micros}us"),
            Time::Nanos(nanos) => write!(f, "{nanos}ns"),
        }
    }
}

impl From<Duration> for Time {
    fn from(value: Duration) -> Self {
        if value.as_secs() > 0 {
            Self::Seconds(value.as_secs_f32())
        } else if value.as_millis() > 0 {
            Self::Millis(value.as_millis())
        } else if value.as_micros() > 0 {
            Self::Micros(value.as_micros())
        } else {
            Self::Nanos(value.as_nanos())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_format() {
        let seconds = Duration::new(2, 123_456_789);
        let millis = Duration::new(0, 123_456_789);
        let micros = Duration::new(0, 456_789);
        let nanos = Duration::new(0, 789);
        assert_eq!("2.123s", format!("{}", Time::from(seconds)));
        assert_eq!("123ms", format!("{}", Time::from(millis)));
        assert_eq!("456us", format!("{}", Time::from(micros)));
        assert_eq!("789ns", format!("{}", Time::from(nanos)));
    }
}
