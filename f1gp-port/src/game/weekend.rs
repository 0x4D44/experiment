//! Race Weekend integration module
//!
//! Ties together Practice, Qualifying, and Race sessions for a complete F1 weekend.
//! Manages the progression between sessions and carries over results.

use super::damage::DamageState;
use super::pitstop::{PitStopManager, RaceStrategy, TireCompound};
use super::qualifying::QualifyingSession;
use super::session::RaceSession;

/// Race weekend session type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekendSession {
    /// Free practice (Friday)
    FreePractice1,
    /// Second practice (Friday)
    FreePractice2,
    /// Third practice (Saturday morning)
    FreePractice3,
    /// Qualifying (Saturday afternoon)
    Qualifying,
    /// Warm-up (Sunday morning, short session)
    WarmUp,
    /// The race (Sunday)
    Race,
}

impl WeekendSession {
    /// Get session name for display
    pub fn name(&self) -> &'static str {
        match self {
            WeekendSession::FreePractice1 => "Free Practice 1",
            WeekendSession::FreePractice2 => "Free Practice 2",
            WeekendSession::FreePractice3 => "Free Practice 3",
            WeekendSession::Qualifying => "Qualifying",
            WeekendSession::WarmUp => "Warm-Up",
            WeekendSession::Race => "Race",
        }
    }

    /// Get short name
    pub fn short_name(&self) -> &'static str {
        match self {
            WeekendSession::FreePractice1 => "FP1",
            WeekendSession::FreePractice2 => "FP2",
            WeekendSession::FreePractice3 => "FP3",
            WeekendSession::Qualifying => "QUALI",
            WeekendSession::WarmUp => "WU",
            WeekendSession::Race => "RACE",
        }
    }

    /// Get session duration in minutes
    pub fn duration_minutes(&self) -> u32 {
        match self {
            WeekendSession::FreePractice1 => 60,
            WeekendSession::FreePractice2 => 60,
            WeekendSession::FreePractice3 => 60,
            WeekendSession::Qualifying => 60,
            WeekendSession::WarmUp => 30,
            WeekendSession::Race => 0, // Race is lap-based, not time-based
        }
    }

    /// Get next session in the weekend
    pub fn next(&self) -> Option<WeekendSession> {
        match self {
            WeekendSession::FreePractice1 => Some(WeekendSession::FreePractice2),
            WeekendSession::FreePractice2 => Some(WeekendSession::FreePractice3),
            WeekendSession::FreePractice3 => Some(WeekendSession::Qualifying),
            WeekendSession::Qualifying => Some(WeekendSession::WarmUp),
            WeekendSession::WarmUp => Some(WeekendSession::Race),
            WeekendSession::Race => None,
        }
    }

    /// Check if this is a practice session
    pub fn is_practice(&self) -> bool {
        matches!(
            self,
            WeekendSession::FreePractice1
                | WeekendSession::FreePractice2
                | WeekendSession::FreePractice3
                | WeekendSession::WarmUp
        )
    }
}

/// Driver entry for the weekend
#[derive(Debug, Clone)]
pub struct WeekendEntry {
    /// Driver name
    pub name: String,
    /// Team name
    pub team: String,
    /// Car number
    pub number: u8,
    /// Grid position (set after qualifying)
    pub grid_position: Option<usize>,
    /// Best practice lap time
    pub best_practice_time: Option<f32>,
    /// Qualifying time
    pub qualifying_time: Option<f32>,
    /// Race finish position
    pub race_position: Option<usize>,
    /// Damage state
    pub damage: DamageState,
    /// Pit stop manager
    pub pit_manager: PitStopManager,
    /// Race strategy
    pub strategy: RaceStrategy,
    /// Car reliability rating
    pub reliability: f32,
}

impl WeekendEntry {
    /// Create a new weekend entry for a driver
    pub fn new(name: &str, team: &str, number: u8, reliability: f32) -> Self {
        Self {
            name: name.to_string(),
            team: team.to_string(),
            number,
            grid_position: None,
            best_practice_time: None,
            qualifying_time: None,
            race_position: None,
            damage: DamageState::new(reliability),
            pit_manager: PitStopManager::new(TireCompound::C),
            strategy: RaceStrategy::one_stop(60), // Default strategy
            reliability,
        }
    }

    /// Reset for a new session
    pub fn reset_for_session(&mut self) {
        self.damage = DamageState::new(self.reliability);
        self.pit_manager = PitStopManager::new(TireCompound::C);
    }
}

/// Race weekend state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekendState {
    /// Not started
    NotStarted,
    /// Session in progress
    InSession,
    /// Between sessions
    BetweenSessions,
    /// Weekend complete
    Complete,
}

/// Complete race weekend manager
pub struct RaceWeekend {
    /// Circuit name
    pub circuit: String,
    /// Track file name
    pub track_file: String,
    /// Number of race laps
    pub race_laps: u32,
    /// Current session
    pub current_session: WeekendSession,
    /// Weekend state
    pub state: WeekendState,
    /// Driver entries
    pub entries: Vec<WeekendEntry>,
    /// Qualifying session (when active)
    pub qualifying: Option<QualifyingSession>,
    /// Race session (when active)
    pub race: Option<RaceSession>,
    /// Reference to championship (if in championship mode)
    championship_round: Option<usize>,
}

impl RaceWeekend {
    /// Create a new race weekend
    pub fn new(circuit: &str, track_file: &str, race_laps: u32) -> Self {
        Self {
            circuit: circuit.to_string(),
            track_file: track_file.to_string(),
            race_laps,
            current_session: WeekendSession::FreePractice1,
            state: WeekendState::NotStarted,
            entries: Vec::new(),
            qualifying: None,
            race: None,
            championship_round: None,
        }
    }

    /// Set championship round
    pub fn set_championship_round(&mut self, round: usize) {
        self.championship_round = Some(round);
    }

    /// Add a driver entry
    pub fn add_entry(&mut self, entry: WeekendEntry) {
        self.entries.push(entry);
    }

    /// Initialize with 1991 F1 grid
    pub fn initialize_1991_grid(&mut self) {
        let drivers = [
            ("Ayrton Senna", "McLaren", 1, 0.92),
            ("Gerhard Berger", "McLaren", 2, 0.90),
            ("Nigel Mansell", "Williams", 5, 0.88),
            ("Riccardo Patrese", "Williams", 6, 0.91),
            ("Alain Prost", "Ferrari", 27, 0.85),
            ("Jean Alesi", "Ferrari", 28, 0.87),
            ("Nelson Piquet", "Benetton", 19, 0.89),
            ("Roberto Moreno", "Benetton", 20, 0.86),
            ("Thierry Boutsen", "Ligier", 25, 0.88),
            ("Erik Comas", "Ligier", 26, 0.84),
            ("Ivan Capelli", "Leyton House", 16, 0.82),
            ("Mauricio Gugelmin", "Leyton House", 15, 0.83),
            ("Stefano Modena", "Tyrrell", 4, 0.85),
            ("Satoru Nakajima", "Tyrrell", 3, 0.84),
            ("Martin Brundle", "Brabham", 7, 0.81),
            ("Mark Blundell", "Brabham", 8, 0.80),
            ("Pierluigi Martini", "Minardi", 23, 0.79),
            ("Gianni Morbidelli", "Minardi", 24, 0.78),
            ("Andrea de Cesaris", "Jordan", 33, 0.83),
            ("Bertrand Gachot", "Jordan", 32, 0.82),
            ("Aguri Suzuki", "Lola", 29, 0.80),
            ("Eric Bernard", "Lola", 30, 0.79),
            ("JJ Lehto", "Dallara", 21, 0.81),
            ("Emanuele Pirro", "Dallara", 22, 0.80),
            ("Michele Alboreto", "Footwork", 9, 0.82),
            ("Alex Caffi", "Footwork", 10, 0.78),
        ];

        for (name, team, number, reliability) in drivers {
            self.add_entry(WeekendEntry::new(name, team, number, reliability));
        }
    }

    /// Start the weekend (begin first session)
    pub fn start(&mut self) {
        self.state = WeekendState::InSession;
        self.current_session = WeekendSession::FreePractice1;
        self.prepare_session();
    }

    /// Skip to a specific session
    pub fn skip_to(&mut self, session: WeekendSession) {
        self.current_session = session;
        self.state = WeekendState::InSession;
        self.prepare_session();
    }

    /// Prepare current session
    fn prepare_session(&mut self) {
        // Reset driver states for new session
        for entry in &mut self.entries {
            entry.reset_for_session();
        }

        match self.current_session {
            WeekendSession::Qualifying => {
                let mut quali = QualifyingSession::new(60);
                for entry in &self.entries {
                    quali.add_driver(&entry.name, &entry.team);
                }
                self.qualifying = Some(quali);
            }
            WeekendSession::Race => {
                // Create race session with grid from qualifying
                let _grid = self.get_grid_order();
                let race = RaceSession::new(self.entries.len(), self.race_laps);

                // Set strategies based on race length
                for entry in &mut self.entries {
                    entry.strategy = if self.race_laps > 40 {
                        RaceStrategy::one_stop(self.race_laps)
                    } else {
                        RaceStrategy::no_stop()
                    };
                }

                self.race = Some(race);
            }
            _ => {
                // Practice sessions don't need special setup
            }
        }
    }

    /// Get grid order (after qualifying)
    pub fn get_grid_order(&self) -> Vec<usize> {
        let mut order: Vec<(usize, Option<f32>)> = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, e)| (i, e.qualifying_time))
            .collect();

        // Sort by qualifying time (None = back of grid)
        order.sort_by(|a, b| match (a.1, b.1) {
            (Some(ta), Some(tb)) => ta.partial_cmp(&tb).unwrap(),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });

        order.iter().map(|(i, _)| *i).collect()
    }

    /// End current session and record results
    pub fn end_session(&mut self) {
        match self.current_session {
            WeekendSession::Qualifying => {
                if let Some(quali) = &self.qualifying {
                    // Copy qualifying results to entries
                    for (i, entry) in self.entries.iter_mut().enumerate() {
                        if let Some(quali_entry) = quali.entries().get(i) {
                            entry.qualifying_time = quali_entry.best_time;
                        }
                    }

                    // Set grid positions
                    let grid = self.get_grid_order();
                    for (pos, &driver_idx) in grid.iter().enumerate() {
                        self.entries[driver_idx].grid_position = Some(pos + 1);
                    }
                }
            }
            WeekendSession::Race => {
                if let Some(race) = &self.race {
                    // Copy race results to entries
                    for result in &race.results {
                        if let Some(entry) = self.entries.iter_mut().find(|e| e.name == result.name)
                        {
                            entry.race_position = Some(result.position);
                        }
                    }
                }
            }
            _ => {
                // Record best practice times
            }
        }

        // Move to next session or complete weekend
        if let Some(next) = self.current_session.next() {
            self.current_session = next;
            self.state = WeekendState::BetweenSessions;
        } else {
            self.state = WeekendState::Complete;
        }
    }

    /// Advance to next session
    pub fn advance_to_next_session(&mut self) {
        if self.state == WeekendState::BetweenSessions {
            self.state = WeekendState::InSession;
            self.prepare_session();
        }
    }

    /// Get race results for championship
    pub fn get_race_results(&self) -> Vec<(String, Option<usize>)> {
        self.entries
            .iter()
            .map(|e| (e.name.clone(), e.race_position))
            .collect()
    }

    /// Get pole sitter name
    pub fn pole_sitter(&self) -> Option<&str> {
        self.entries
            .iter()
            .find(|e| e.grid_position == Some(1))
            .map(|e| e.name.as_str())
    }

    /// Get race winner name
    pub fn race_winner(&self) -> Option<&str> {
        self.entries
            .iter()
            .find(|e| e.race_position == Some(1))
            .map(|e| e.name.as_str())
    }

    /// Check if weekend is complete
    pub fn is_complete(&self) -> bool {
        self.state == WeekendState::Complete
    }

    /// Get number of entries
    pub fn num_entries(&self) -> usize {
        self.entries.len()
    }
}

/// Create a race weekend for a specific track
pub fn create_weekend(circuit: &str, track_file: &str, race_laps: u32) -> RaceWeekend {
    let mut weekend = RaceWeekend::new(circuit, track_file, race_laps);
    weekend.initialize_1991_grid();
    weekend
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weekend_session_progression() {
        assert_eq!(
            WeekendSession::FreePractice1.next(),
            Some(WeekendSession::FreePractice2)
        );
        assert_eq!(
            WeekendSession::Qualifying.next(),
            Some(WeekendSession::WarmUp)
        );
        assert_eq!(WeekendSession::Race.next(), None);
    }

    #[test]
    fn test_weekend_session_names() {
        assert_eq!(WeekendSession::FreePractice1.name(), "Free Practice 1");
        assert_eq!(WeekendSession::Qualifying.short_name(), "QUALI");
    }

    #[test]
    fn test_weekend_entry_creation() {
        let entry = WeekendEntry::new("Test Driver", "Test Team", 1, 0.9);
        assert_eq!(entry.name, "Test Driver");
        assert_eq!(entry.number, 1);
        assert!(entry.grid_position.is_none());
    }

    #[test]
    fn test_race_weekend_creation() {
        let weekend = RaceWeekend::new("Monaco", "F1CT04.DAT", 78);
        assert_eq!(weekend.circuit, "Monaco");
        assert_eq!(weekend.race_laps, 78);
        assert_eq!(weekend.state, WeekendState::NotStarted);
    }

    #[test]
    fn test_weekend_with_1991_grid() {
        let weekend = create_weekend("Monaco", "F1CT04.DAT", 78);
        assert_eq!(weekend.num_entries(), 26);
    }

    #[test]
    fn test_weekend_start() {
        let mut weekend = create_weekend("Monaco", "F1CT04.DAT", 78);
        weekend.start();

        assert_eq!(weekend.state, WeekendState::InSession);
        assert_eq!(weekend.current_session, WeekendSession::FreePractice1);
    }

    #[test]
    fn test_skip_to_qualifying() {
        let mut weekend = create_weekend("Monaco", "F1CT04.DAT", 78);
        weekend.skip_to(WeekendSession::Qualifying);

        assert_eq!(weekend.current_session, WeekendSession::Qualifying);
        assert!(weekend.qualifying.is_some());
    }

    #[test]
    fn test_grid_order_no_times() {
        let weekend = create_weekend("Monaco", "F1CT04.DAT", 78);
        let grid = weekend.get_grid_order();
        assert_eq!(grid.len(), 26);
    }

    #[test]
    fn test_grid_order_with_times() {
        let mut weekend = create_weekend("Monaco", "F1CT04.DAT", 78);

        // Set some qualifying times
        weekend.entries[0].qualifying_time = Some(90.0);
        weekend.entries[1].qualifying_time = Some(88.0);
        weekend.entries[2].qualifying_time = Some(92.0);

        let grid = weekend.get_grid_order();

        // Driver with 88.0 should be first
        assert_eq!(grid[0], 1);
        assert_eq!(grid[1], 0);
        assert_eq!(grid[2], 2);
    }

    #[test]
    fn test_session_end() {
        let mut weekend = create_weekend("Monaco", "F1CT04.DAT", 78);
        weekend.start();
        weekend.end_session();

        assert_eq!(weekend.state, WeekendState::BetweenSessions);
        assert_eq!(weekend.current_session, WeekendSession::FreePractice2);
    }

    #[test]
    fn test_weekend_complete() {
        let mut weekend = create_weekend("Monaco", "F1CT04.DAT", 78);
        weekend.skip_to(WeekendSession::Race);
        weekend.end_session();

        assert!(weekend.is_complete());
    }

    #[test]
    fn test_is_practice() {
        assert!(WeekendSession::FreePractice1.is_practice());
        assert!(WeekendSession::WarmUp.is_practice());
        assert!(!WeekendSession::Qualifying.is_practice());
        assert!(!WeekendSession::Race.is_practice());
    }
}
