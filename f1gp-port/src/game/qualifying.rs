//! Qualifying session management
//!
//! Handles timed qualifying sessions where drivers compete for grid positions.
//! F1GP uses a single 1-hour qualifying session where each driver can do unlimited laps.

/// Qualifying session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualifyingState {
    /// Pre-session, cars in garage
    PreSession,

    /// Session is green, cars can set times
    Green,

    /// Session paused (red flag)
    RedFlag,

    /// Session complete
    Finished,
}

/// Individual driver qualifying entry
#[derive(Debug, Clone)]
pub struct QualifyingEntry {
    /// Driver name
    pub name: String,

    /// Team name
    pub team: String,

    /// Best lap time (None if no time set)
    pub best_time: Option<f32>,

    /// All lap times set during qualifying
    pub lap_times: Vec<f32>,

    /// Number of laps attempted
    pub laps_attempted: u32,

    /// Current out lap (not a flying lap)
    pub on_out_lap: bool,

    /// Currently in pit lane
    pub in_pits: bool,

    /// Sector 1 time of current lap
    pub sector1_time: Option<f32>,

    /// Sector 2 time of current lap
    pub sector2_time: Option<f32>,

    /// Current lap start time (session time)
    pub lap_start_time: f32,
}

impl QualifyingEntry {
    /// Create a new qualifying entry
    pub fn new(name: &str, team: &str) -> Self {
        Self {
            name: name.to_string(),
            team: team.to_string(),
            best_time: None,
            lap_times: Vec::new(),
            laps_attempted: 0,
            on_out_lap: true,
            in_pits: true,
            sector1_time: None,
            sector2_time: None,
            lap_start_time: 0.0,
        }
    }

    /// Record a completed lap time
    pub fn record_lap(&mut self, time: f32) {
        self.lap_times.push(time);
        self.laps_attempted += 1;

        // Update best time if this is faster
        if self.best_time.is_none() || time < self.best_time.unwrap() {
            self.best_time = Some(time);
        }

        // Reset sector times for next lap
        self.sector1_time = None;
        self.sector2_time = None;
        self.on_out_lap = false;
    }

    /// Get improvement over previous best (negative = faster)
    pub fn improvement(&self) -> Option<f32> {
        if self.lap_times.len() < 2 {
            return None;
        }

        let current = self.lap_times.last()?;
        let previous_best = self.lap_times[..self.lap_times.len() - 1]
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())?;

        Some(*current - previous_best)
    }
}

/// Qualifying session result
#[derive(Debug, Clone)]
pub struct QualifyingResult {
    /// Driver name
    pub name: String,

    /// Team name
    pub team: String,

    /// Grid position (1-based)
    pub position: usize,

    /// Best qualifying time
    pub best_time: Option<f32>,

    /// Gap to pole position (seconds)
    pub gap_to_pole: f32,

    /// Gap to car ahead (seconds)
    pub gap_to_ahead: f32,

    /// Number of laps completed
    pub laps: u32,
}

/// Qualifying session manager
pub struct QualifyingSession {
    /// Current session state
    pub state: QualifyingState,

    /// Session duration (seconds) - typically 3600 (1 hour)
    pub session_duration: f32,

    /// Time elapsed in session
    pub session_time: f32,

    /// Time remaining
    pub time_remaining: f32,

    /// Driver entries
    entries: Vec<QualifyingEntry>,

    /// Provisional grid order (driver indices sorted by best time)
    provisional_grid: Vec<usize>,

    /// Final results (populated when session ends)
    pub results: Vec<QualifyingResult>,

    /// Track record (for comparison)
    pub track_record: Option<f32>,

    /// Session best time (current pole)
    pub session_best: Option<f32>,
}

impl QualifyingSession {
    /// Create a new qualifying session
    ///
    /// # Arguments
    /// * `duration_minutes` - Session duration in minutes (typically 60)
    pub fn new(duration_minutes: u32) -> Self {
        let duration_seconds = duration_minutes as f32 * 60.0;

        Self {
            state: QualifyingState::PreSession,
            session_duration: duration_seconds,
            session_time: 0.0,
            time_remaining: duration_seconds,
            entries: Vec::new(),
            provisional_grid: Vec::new(),
            results: Vec::new(),
            track_record: None,
            session_best: None,
        }
    }

    /// Add a driver to the qualifying session
    pub fn add_driver(&mut self, name: &str, team: &str) {
        self.entries.push(QualifyingEntry::new(name, team));
        self.provisional_grid.push(self.entries.len() - 1);
    }

    /// Initialize with a list of drivers
    pub fn initialize_drivers(&mut self, drivers: Vec<(String, String)>) {
        for (name, team) in drivers {
            self.add_driver(&name, &team);
        }
    }

    /// Start the qualifying session
    pub fn start_session(&mut self) {
        self.state = QualifyingState::Green;
        self.session_time = 0.0;
        self.time_remaining = self.session_duration;
    }

    /// Update the session
    pub fn update(&mut self, delta_time: f32) {
        match self.state {
            QualifyingState::PreSession => {
                // Waiting to start
            }

            QualifyingState::Green => {
                self.session_time += delta_time;
                self.time_remaining = (self.session_duration - self.session_time).max(0.0);

                // Check if session should end
                if self.time_remaining <= 0.0 {
                    self.finish_session();
                }
            }

            QualifyingState::RedFlag => {
                // Session paused, time doesn't advance
            }

            QualifyingState::Finished => {
                // Session over
            }
        }
    }

    /// Driver leaves pit lane to start a lap
    pub fn driver_leave_pits(&mut self, driver_index: usize) {
        if self.state != QualifyingState::Green {
            return;
        }

        if let Some(entry) = self.entries.get_mut(driver_index) {
            entry.in_pits = false;
            entry.on_out_lap = true;
            entry.lap_start_time = self.session_time;
            entry.sector1_time = None;
            entry.sector2_time = None;
        }
    }

    /// Driver enters pit lane
    pub fn driver_enter_pits(&mut self, driver_index: usize) {
        if let Some(entry) = self.entries.get_mut(driver_index) {
            entry.in_pits = true;
            entry.on_out_lap = true;
        }
    }

    /// Record sector 1 time
    pub fn record_sector1(&mut self, driver_index: usize, sector_time: f32) {
        if let Some(entry) = self.entries.get_mut(driver_index) {
            entry.sector1_time = Some(sector_time);
        }
    }

    /// Record sector 2 time
    pub fn record_sector2(&mut self, driver_index: usize, sector_time: f32) {
        if let Some(entry) = self.entries.get_mut(driver_index) {
            entry.sector2_time = Some(sector_time);
        }
    }

    /// Driver crosses start/finish line (completes a lap)
    pub fn complete_lap(&mut self, driver_index: usize) {
        if self.state != QualifyingState::Green {
            return;
        }

        if let Some(entry) = self.entries.get_mut(driver_index) {
            // Skip out laps (they're not flying laps)
            if entry.on_out_lap {
                entry.on_out_lap = false;
                entry.lap_start_time = self.session_time;
                return;
            }

            // Calculate lap time
            let lap_time = self.session_time - entry.lap_start_time;

            // Validate lap time (must be reasonable)
            if lap_time < 30.0 {
                // Too fast, ignore (probably a pit lane exit)
                return;
            }

            // Record the lap
            entry.record_lap(lap_time);

            // Update session best
            if self.session_best.is_none() || lap_time < self.session_best.unwrap() {
                self.session_best = Some(lap_time);
            }

            // Start next lap
            entry.lap_start_time = self.session_time;

            // Update provisional grid
            self.update_provisional_grid();
        }
    }

    /// Update provisional grid order based on best times
    fn update_provisional_grid(&mut self) {
        // Sort driver indices by best time (None times go to the back)
        self.provisional_grid.sort_by(|&a, &b| {
            let time_a = self.entries.get(a).and_then(|e| e.best_time);
            let time_b = self.entries.get(b).and_then(|e| e.best_time);

            match (time_a, time_b) {
                (Some(ta), Some(tb)) => ta.partial_cmp(&tb).unwrap(),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
    }

    /// Get current grid position for a driver (1-based)
    pub fn get_grid_position(&self, driver_index: usize) -> usize {
        self.provisional_grid
            .iter()
            .position(|&idx| idx == driver_index)
            .map(|p| p + 1)
            .unwrap_or(self.entries.len())
    }

    /// Get the current pole sitter
    pub fn pole_sitter(&self) -> Option<&QualifyingEntry> {
        self.provisional_grid
            .first()
            .and_then(|&idx| self.entries.get(idx))
    }

    /// Get gap to pole for a driver
    pub fn gap_to_pole(&self, driver_index: usize) -> Option<f32> {
        let pole_time = self.session_best?;
        let driver_time = self.entries.get(driver_index)?.best_time?;
        Some(driver_time - pole_time)
    }

    /// Trigger red flag (pause session)
    pub fn red_flag(&mut self) {
        if self.state == QualifyingState::Green {
            self.state = QualifyingState::RedFlag;
        }
    }

    /// Resume from red flag
    pub fn resume_session(&mut self) {
        if self.state == QualifyingState::RedFlag {
            self.state = QualifyingState::Green;
        }
    }

    /// End the session and compute final results
    pub fn finish_session(&mut self) {
        self.state = QualifyingState::Finished;
        self.update_provisional_grid();

        // Build final results
        let mut results = Vec::new();
        let pole_time = self.session_best;

        for (position, &driver_idx) in self.provisional_grid.iter().enumerate() {
            if let Some(entry) = self.entries.get(driver_idx) {
                let gap_to_pole = match (pole_time, entry.best_time) {
                    (Some(pt), Some(bt)) => bt - pt,
                    _ => 0.0,
                };

                let gap_to_ahead = if position == 0 {
                    0.0
                } else {
                    let ahead_idx = self.provisional_grid[position - 1];
                    match (
                        self.entries.get(ahead_idx).and_then(|e| e.best_time),
                        entry.best_time,
                    ) {
                        (Some(at), Some(bt)) => bt - at,
                        _ => 0.0,
                    }
                };

                results.push(QualifyingResult {
                    name: entry.name.clone(),
                    team: entry.team.clone(),
                    position: position + 1,
                    best_time: entry.best_time,
                    gap_to_pole,
                    gap_to_ahead,
                    laps: entry.laps_attempted,
                });
            }
        }

        self.results = results;
    }

    /// Get all entries (for display)
    pub fn entries(&self) -> &[QualifyingEntry] {
        &self.entries
    }

    /// Get mutable entry
    pub fn entry_mut(&mut self, driver_index: usize) -> Option<&mut QualifyingEntry> {
        self.entries.get_mut(driver_index)
    }

    /// Get number of drivers
    pub fn num_drivers(&self) -> usize {
        self.entries.len()
    }

    /// Get the final grid order (driver indices)
    pub fn grid_order(&self) -> &[usize] {
        &self.provisional_grid
    }

    /// Check if session is active
    pub fn is_active(&self) -> bool {
        self.state == QualifyingState::Green
    }

    /// Format time as MM:SS.mmm
    pub fn format_time(seconds: f32) -> String {
        let mins = (seconds / 60.0).floor() as u32;
        let secs = seconds % 60.0;
        format!("{}:{:06.3}", mins, secs)
    }

    /// Format time remaining as MM:SS
    pub fn format_time_remaining(&self) -> String {
        let mins = (self.time_remaining / 60.0).floor() as u32;
        let secs = (self.time_remaining % 60.0).floor() as u32;
        format!("{:02}:{:02}", mins, secs)
    }
}

/// Create a default 60-minute qualifying session with 1991 drivers
pub fn create_1991_qualifying() -> QualifyingSession {
    let mut session = QualifyingSession::new(60);

    let drivers = vec![
        ("Ayrton Senna", "McLaren"),
        ("Gerhard Berger", "McLaren"),
        ("Nigel Mansell", "Williams"),
        ("Riccardo Patrese", "Williams"),
        ("Alain Prost", "Ferrari"),
        ("Jean Alesi", "Ferrari"),
        ("Nelson Piquet", "Benetton"),
        ("Roberto Moreno", "Benetton"),
        ("Thierry Boutsen", "Ligier"),
        ("Erik Comas", "Ligier"),
        ("Ivan Capelli", "Leyton House"),
        ("Mauricio Gugelmin", "Leyton House"),
        ("Stefano Modena", "Tyrrell"),
        ("Satoru Nakajima", "Tyrrell"),
        ("Martin Brundle", "Brabham"),
        ("Mark Blundell", "Brabham"),
        ("Pierluigi Martini", "Minardi"),
        ("Gianni Morbidelli", "Minardi"),
        ("Andrea de Cesaris", "Jordan"),
        ("Bertrand Gachot", "Jordan"),
        ("Aguri Suzuki", "Lola"),
        ("Eric Bernard", "Lola"),
        ("JJ Lehto", "Dallara"),
        ("Emanuele Pirro", "Dallara"),
        ("Michele Alboreto", "Footwork"),
        ("Alex Caffi", "Footwork"),
    ];

    for (name, team) in drivers {
        session.add_driver(name, team);
    }

    session
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qualifying_session_creation() {
        let session = QualifyingSession::new(60);
        assert_eq!(session.session_duration, 3600.0);
        assert_eq!(session.state, QualifyingState::PreSession);
    }

    #[test]
    fn test_add_drivers() {
        let mut session = QualifyingSession::new(60);
        session.add_driver("Test Driver", "Test Team");

        assert_eq!(session.num_drivers(), 1);
        assert_eq!(session.entries[0].name, "Test Driver");
    }

    #[test]
    fn test_1991_qualifying() {
        let session = create_1991_qualifying();
        assert_eq!(session.num_drivers(), 26);
    }

    #[test]
    fn test_lap_recording() {
        let mut entry = QualifyingEntry::new("Test", "Team");

        entry.record_lap(92.5);
        assert_eq!(entry.best_time, Some(92.5));
        assert_eq!(entry.laps_attempted, 1);

        entry.record_lap(91.8);
        assert_eq!(entry.best_time, Some(91.8));
        assert_eq!(entry.laps_attempted, 2);

        entry.record_lap(93.2);
        assert_eq!(entry.best_time, Some(91.8)); // Still best
    }

    #[test]
    fn test_session_timing() {
        let mut session = QualifyingSession::new(60);
        session.add_driver("Test", "Team");
        session.start_session();

        assert_eq!(session.state, QualifyingState::Green);
        assert_eq!(session.time_remaining, 3600.0);

        session.update(60.0); // 1 minute
        assert_eq!(session.time_remaining, 3540.0);
    }

    #[test]
    fn test_lap_completion() {
        let mut session = QualifyingSession::new(60);
        session.add_driver("Driver 1", "Team 1");
        session.add_driver("Driver 2", "Team 2");
        session.start_session();

        // Driver leaves pits
        session.driver_leave_pits(0);
        assert!(!session.entries[0].in_pits);
        assert!(session.entries[0].on_out_lap);

        // Complete out lap
        session.session_time = 120.0; // 2 minutes
        session.complete_lap(0);
        assert!(!session.entries[0].on_out_lap);

        // Complete flying lap
        session.session_time = 212.5; // ~92.5 seconds later
        session.complete_lap(0);

        assert_eq!(session.entries[0].laps_attempted, 1);
        assert!(session.entries[0].best_time.is_some());
    }

    #[test]
    fn test_grid_order() {
        let mut session = QualifyingSession::new(60);
        session.add_driver("Slow Driver", "Team 1");
        session.add_driver("Fast Driver", "Team 2");
        session.start_session();

        // Fast driver sets time
        session.entries[1].record_lap(88.5);
        session.update_provisional_grid();

        // Slow driver sets time
        session.entries[0].record_lap(92.0);
        session.update_provisional_grid();

        // Fast driver should be P1
        assert_eq!(session.get_grid_position(1), 1);
        assert_eq!(session.get_grid_position(0), 2);
    }

    #[test]
    fn test_gap_to_pole() {
        let mut session = QualifyingSession::new(60);
        session.add_driver("P1", "Team 1");
        session.add_driver("P2", "Team 2");
        session.start_session();

        session.entries[0].record_lap(90.0);
        session.entries[1].record_lap(91.5);
        session.session_best = Some(90.0);
        session.update_provisional_grid();

        let gap = session.gap_to_pole(1);
        assert_eq!(gap, Some(1.5));
    }

    #[test]
    fn test_session_finish() {
        let mut session = QualifyingSession::new(1); // 1 minute session
        session.add_driver("Driver", "Team");
        session.start_session();

        session.entries[0].record_lap(45.0);
        session.session_best = Some(45.0);

        // Advance past session end
        session.update(61.0);

        assert_eq!(session.state, QualifyingState::Finished);
        assert_eq!(session.results.len(), 1);
        assert_eq!(session.results[0].position, 1);
    }

    #[test]
    fn test_red_flag() {
        let mut session = QualifyingSession::new(60);
        session.start_session();

        session.update(10.0);
        let time_before = session.session_time;

        session.red_flag();
        assert_eq!(session.state, QualifyingState::RedFlag);

        session.update(10.0);
        assert_eq!(session.session_time, time_before); // Time frozen

        session.resume_session();
        assert_eq!(session.state, QualifyingState::Green);
    }

    #[test]
    fn test_format_time() {
        assert_eq!(QualifyingSession::format_time(92.567), "1:32.567");
        assert_eq!(QualifyingSession::format_time(65.123), "1:05.123");
        assert_eq!(QualifyingSession::format_time(45.0), "0:45.000");
    }

    #[test]
    fn test_improvement() {
        let mut entry = QualifyingEntry::new("Test", "Team");

        entry.record_lap(92.5);
        assert_eq!(entry.improvement(), None); // No previous lap

        entry.record_lap(91.8);
        let improvement = entry.improvement().unwrap();
        assert!(improvement < 0.0); // Improved (negative = faster)
        assert!((improvement - (-0.7)).abs() < 0.01); // Approximately -0.7s

        entry.record_lap(93.0);
        assert!(entry.improvement().unwrap() > 0.0); // Slower
    }
}
