//! Game timer with millisecond precision

use std::time::Instant;
use std::fmt;

/// Game timer that tracks elapsed time
#[derive(Debug, Clone)]
pub struct GameTimer {
    start_time: Option<Instant>,
    paused_time: u64,
    is_paused: bool,
}

impl GameTimer {
    /// Create a new timer
    pub fn new() -> Self {
        GameTimer {
            start_time: None,
            paused_time: 0,
            is_paused: false,
        }
    }

    /// Start the timer
    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    /// Stop/pause the timer
    pub fn stop(&mut self) {
        if let Some(start) = self.start_time {
            self.paused_time = start.elapsed().as_millis() as u64;
            self.is_paused = true;
        }
    }

    /// Resume the timer
    pub fn resume(&mut self) {
        if self.is_paused {
            self.start_time = Some(Instant::now() - std::time::Duration::from_millis(self.paused_time));
            self.is_paused = false;
        }
    }

    /// Get elapsed time in milliseconds
    pub fn elapsed_ms(&self) -> u64 {
        if let Some(start) = self.start_time {
            if self.is_paused {
                self.paused_time
            } else {
                start.elapsed().as_millis() as u64
            }
        } else {
            0
        }
    }

    /// Get elapsed time formatted as MM:SS.d
    pub fn formatted(&self) -> String {
        let ms = self.elapsed_ms();
        let total_seconds = ms / 1000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        let deciseconds = (ms % 1000) / 100;

        format!("{:02}:{:02}.{}", minutes, seconds, deciseconds)
    }

    /// Get elapsed time as a Duration
    pub fn duration(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.elapsed_ms())
    }

    /// Check if timer is running
    pub fn is_running(&self) -> bool {
        self.start_time.is_some() && !self.is_paused
    }

    /// Reset timer
    pub fn reset(&mut self) {
        self.start_time = None;
        self.paused_time = 0;
        self.is_paused = false;
    }
}

impl Default for GameTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GameTimer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

impl PartialEq for GameTimer {
    fn eq(&self, other: &Self) -> bool {
        self.elapsed_ms() == other.elapsed_ms()
    }
}

impl PartialOrd for GameTimer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.elapsed_ms().partial_cmp(&other.elapsed_ms())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_timer_creation() {
        let timer = GameTimer::new();
        assert!(!timer.is_running());
        assert_eq!(timer.elapsed_ms(), 0);
    }

    #[test]
    fn test_timer_start() {
        let mut timer = GameTimer::new();
        timer.start();
        assert!(timer.is_running());
    }

    #[test]
    fn test_timer_advances() {
        let mut timer = GameTimer::new();
        timer.start();

        let time1 = timer.elapsed_ms();
        thread::sleep(Duration::from_millis(50));
        let time2 = timer.elapsed_ms();

        assert!(time2 > time1);
    }

    #[test]
    fn test_timer_stop() {
        let mut timer = GameTimer::new();
        timer.start();
        thread::sleep(Duration::from_millis(50));

        timer.stop();
        assert!(!timer.is_running());

        let frozen_time = timer.elapsed_ms();
        thread::sleep(Duration::from_millis(50));

        // Time should not change after stop
        assert_eq!(timer.elapsed_ms(), frozen_time);
    }

    #[test]
    fn test_timer_formatted() {
        let mut timer = GameTimer::new();
        timer.start();

        thread::sleep(Duration::from_millis(2500)); // 2.5 seconds

        let formatted = timer.formatted();
        assert!(formatted.contains("00:0"));
        assert!(formatted.contains("."));
    }

    #[test]
    fn test_timer_formatted_minutes() {
        let mut timer = GameTimer::new();
        timer.start();

        // Manually set time
        thread::sleep(Duration::from_millis(65000)); // 1 minute 5 seconds

        let formatted = timer.formatted();
        assert!(formatted.starts_with("01:"));
    }

    #[test]
    fn test_timer_reset() {
        let mut timer = GameTimer::new();
        timer.start();
        thread::sleep(Duration::from_millis(50));

        timer.reset();
        assert_eq!(timer.elapsed_ms(), 0);
        assert!(!timer.is_running());
    }

    #[test]
    fn test_timer_comparison() {
        let mut timer1 = GameTimer::new();
        let mut timer2 = GameTimer::new();

        timer1.start();
        timer2.start();

        // Small time difference won't be noticeable
        thread::sleep(Duration::from_millis(10));

        assert!(timer1.elapsed_ms() > 0);
        assert!(timer2.elapsed_ms() > 0);
    }
}
