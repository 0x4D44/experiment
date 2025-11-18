//! Race session management
//!
//! Handles race start sequences, flags, lap counting, and race completion.

/// Race session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaceState {
    /// Pre-race grid formation
    GridFormation,

    /// Countdown sequence (5 red lights)
    Countdown,

    /// Race is active
    Racing,

    /// Race finished (checkered flag)
    Finished,

    /// Race aborted
    Aborted,
}

/// Flag conditions during the race
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaceFlag {
    /// Green flag - racing conditions
    Green,

    /// Yellow flag - caution, no overtaking
    Yellow,

    /// Blue flag - being lapped, let faster car pass
    Blue,

    /// Checkered flag - race finished
    Checkered,

    /// Red flag - race stopped
    Red,
}

/// Driver race result
#[derive(Debug, Clone)]
pub struct DriverResult {
    /// Driver name
    pub name: String,

    /// Final position (1-based)
    pub position: usize,

    /// Total race time (seconds)
    pub race_time: f32,

    /// Laps completed
    pub laps_completed: u32,

    /// Best lap time (seconds)
    pub best_lap: Option<f32>,

    /// Did the driver finish the race?
    pub finished: bool,

    /// Did Not Finish reason (if applicable)
    pub dnf_reason: Option<String>,
}

/// Race session manager
pub struct RaceSession {
    /// Current race state
    pub state: RaceState,

    /// Current flag condition
    pub flag: RaceFlag,

    /// Total number of laps in the race
    pub total_laps: u32,

    /// Grid positions (driver index -> grid position)
    _grid_positions: Vec<usize>,

    /// Countdown timer (seconds)
    countdown_timer: f32,

    /// Current red lights (0-5 during countdown)
    red_lights: u32,

    /// Time since race start (seconds)
    race_time: f32,

    /// Driver lap counts (driver index -> laps completed)
    driver_laps: Vec<u32>,

    /// Driver lap times (driver index -> vec of lap times)
    driver_lap_times: Vec<Vec<f32>>,

    /// Current lap start times (driver index -> lap start time)
    lap_start_times: Vec<f32>,

    /// Race results (populated when race finishes)
    pub results: Vec<DriverResult>,
}

impl RaceSession {
    /// Create a new race session
    pub fn new(num_drivers: usize, total_laps: u32) -> Self {
        Self {
            state: RaceState::GridFormation,
            flag: RaceFlag::Green,
            total_laps,
            _grid_positions: (0..num_drivers).collect(),
            countdown_timer: 0.0,
            red_lights: 0,
            race_time: 0.0,
            driver_laps: vec![0; num_drivers],
            driver_lap_times: vec![Vec::new(); num_drivers],
            lap_start_times: vec![0.0; num_drivers],
            results: Vec::new(),
        }
    }

    /// Start the race countdown sequence
    pub fn start_countdown(&mut self) {
        self.state = RaceState::Countdown;
        self.countdown_timer = 0.0;
        self.red_lights = 0;
    }

    /// Update the race session
    pub fn update(&mut self, delta_time: f32, driver_names: &[String]) {
        match self.state {
            RaceState::GridFormation => {
                // Waiting for race to start
            }

            RaceState::Countdown => {
                self.countdown_timer += delta_time;

                // Light up red lights every second (5 lights total)
                let lights = (self.countdown_timer / 1.0).floor() as u32;
                self.red_lights = lights.min(5);

                // All lights on and time elapsed? Start race!
                if self.red_lights >= 5 && self.countdown_timer >= 6.0 {
                    // Lights out and away we go!
                    self.state = RaceState::Racing;
                    self.race_time = 0.0;

                    // Initialize lap timers
                    for i in 0..self.lap_start_times.len() {
                        self.lap_start_times[i] = 0.0;
                    }
                }
            }

            RaceState::Racing => {
                self.race_time += delta_time;

                // Check if race should finish (leader completed all laps)
                if let Some(max_laps) = self.driver_laps.iter().max() {
                    if *max_laps >= self.total_laps {
                        self.finish_race(driver_names);
                    }
                }
            }

            RaceState::Finished | RaceState::Aborted => {
                // Race over, do nothing
            }
        }
    }

    /// Record a lap completion for a driver
    pub fn complete_lap(&mut self, driver_index: usize) {
        if self.state != RaceState::Racing {
            return;
        }

        if driver_index >= self.driver_laps.len() {
            return;
        }

        // Calculate lap time
        let lap_time = self.race_time - self.lap_start_times[driver_index];

        // Don't record laps that are too short (< 1 second = false lap)
        if lap_time < 1.0 {
            return;
        }

        // Increment lap count
        self.driver_laps[driver_index] += 1;

        // Record lap time
        self.driver_lap_times[driver_index].push(lap_time);

        // Update lap start time
        self.lap_start_times[driver_index] = self.race_time;
    }

    /// Get the current lap for a driver
    pub fn get_driver_lap(&self, driver_index: usize) -> u32 {
        self.driver_laps.get(driver_index).copied().unwrap_or(0)
    }

    /// Get the best lap time for a driver
    pub fn get_best_lap(&self, driver_index: usize) -> Option<f32> {
        self.driver_lap_times
            .get(driver_index)?
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Get current lap time for a driver
    pub fn get_current_lap_time(&self, driver_index: usize) -> f32 {
        if self.state != RaceState::Racing {
            return 0.0;
        }

        self.race_time - self.lap_start_times.get(driver_index).copied().unwrap_or(0.0)
    }

    /// Determine flag for a specific driver
    pub fn get_driver_flag(&self, driver_index: usize) -> RaceFlag {
        if self.state == RaceState::Finished {
            return RaceFlag::Checkered;
        }

        // Check if being lapped (leader is 3+ laps ahead)
        if let Some(max_laps) = self.driver_laps.iter().max() {
            let driver_laps = self.driver_laps.get(driver_index).copied().unwrap_or(0);
            if *max_laps > driver_laps + 2 {
                return RaceFlag::Blue;
            }
        }

        self.flag
    }

    /// Finish the race and calculate results
    fn finish_race(&mut self, driver_names: &[String]) {
        self.state = RaceState::Finished;
        self.flag = RaceFlag::Checkered;

        // Build results
        let mut results = Vec::new();

        for (i, name) in driver_names.iter().enumerate() {
            let laps_completed = self.driver_laps.get(i).copied().unwrap_or(0);
            let best_lap = self.get_best_lap(i);

            // Calculate total race time
            let race_time = if laps_completed >= self.total_laps {
                // Finished: sum of all lap times
                self.driver_lap_times.get(i)
                    .map(|times| times.iter().sum())
                    .unwrap_or(0.0)
            } else {
                // DNF: time until they stopped
                self.race_time
            };

            results.push(DriverResult {
                name: name.clone(),
                position: 0, // Will be set after sorting
                race_time,
                laps_completed,
                best_lap,
                finished: laps_completed >= self.total_laps,
                dnf_reason: if laps_completed < self.total_laps {
                    Some("Did not complete race".to_string())
                } else {
                    None
                },
            });
        }

        // Sort by: laps completed (desc), then race time (asc)
        results.sort_by(|a, b| {
            b.laps_completed.cmp(&a.laps_completed)
                .then(a.race_time.partial_cmp(&b.race_time).unwrap())
        });

        // Assign positions
        for (i, result) in results.iter_mut().enumerate() {
            result.position = i + 1;
        }

        self.results = results;
    }

    /// Get the current red lights state (for rendering)
    pub fn get_red_lights(&self) -> u32 {
        if self.state == RaceState::Countdown {
            self.red_lights
        } else {
            0
        }
    }

    /// Check if race is in countdown
    pub fn is_countdown(&self) -> bool {
        self.state == RaceState::Countdown
    }

    /// Check if lights should be shown (countdown and lights still on)
    pub fn show_lights(&self) -> bool {
        self.state == RaceState::Countdown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_session_creation() {
        let session = RaceSession::new(6, 10);
        assert_eq!(session.state, RaceState::GridFormation);
        assert_eq!(session.flag, RaceFlag::Green);
        assert_eq!(session.total_laps, 10);
        assert_eq!(session.driver_laps.len(), 6);
    }

    #[test]
    fn test_countdown_sequence() {
        let mut session = RaceSession::new(6, 10);
        session.start_countdown();

        assert_eq!(session.state, RaceState::Countdown);
        assert_eq!(session.red_lights, 0);

        // Advance 3 seconds
        session.update(3.0, &vec![]);
        assert_eq!(session.red_lights, 3);

        // Advance to lights out
        session.update(3.0, &vec![]);
        assert_eq!(session.state, RaceState::Racing);
    }

    #[test]
    fn test_lap_completion() {
        let mut session = RaceSession::new(2, 5);
        session.state = RaceState::Racing;

        // Complete a lap after 90 seconds
        session.race_time = 90.0;
        session.complete_lap(0);

        assert_eq!(session.get_driver_lap(0), 1);
        assert_eq!(session.driver_lap_times[0].len(), 1);
        assert_eq!(session.driver_lap_times[0][0], 90.0);
    }

    #[test]
    fn test_best_lap() {
        let mut session = RaceSession::new(1, 5);
        session.driver_lap_times[0] = vec![92.5, 88.3, 90.1];

        let best = session.get_best_lap(0);
        assert_eq!(best, Some(88.3));
    }

    #[test]
    fn test_blue_flag() {
        let mut session = RaceSession::new(3, 10);
        session.state = RaceState::Racing;
        session.driver_laps = vec![5, 3, 2]; // Leader 5 laps, others behind

        // Driver 2 is being lapped (more than 1 lap behind)
        let flag = session.get_driver_flag(2);
        assert_eq!(flag, RaceFlag::Blue);

        // Driver 1 is not being lapped (only 2 laps behind)
        let flag = session.get_driver_flag(1);
        assert_eq!(flag, RaceFlag::Green);
    }

    #[test]
    fn test_race_finish() {
        let mut session = RaceSession::new(2, 3);
        session.state = RaceState::Racing;

        let names = vec!["Driver 1".to_string(), "Driver 2".to_string()];

        // Driver 0 completes 3 laps
        session.driver_laps[0] = 3;
        session.driver_lap_times[0] = vec![90.0, 88.5, 89.2];

        // Driver 1 completes 2 laps
        session.driver_laps[1] = 2;
        session.driver_lap_times[1] = vec![92.0, 90.5];

        session.update(0.016, &names);

        assert_eq!(session.state, RaceState::Finished);
        assert_eq!(session.results.len(), 2);
        assert_eq!(session.results[0].position, 1);
        assert_eq!(session.results[0].name, "Driver 1");
        assert!(session.results[0].finished);
        assert!(!session.results[1].finished);
    }
}
